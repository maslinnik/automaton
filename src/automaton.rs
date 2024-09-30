use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub trait Symbol: Eq + Clone + Hash + 'static {}

#[derive(Clone)]
pub struct Transition<S: Symbol> {
    next_state: usize,
    symbol: Option<S>,
}

impl<S: Symbol> Transition<S> {
    fn empty(next_state: usize) -> Transition<S> {
        Transition { next_state, symbol: None }
    }

    fn single_symbol(symbol: S, next_state: usize) -> Transition<S> {
        Transition { next_state, symbol: Some(symbol) }
    }

    fn next<'a>(&self, word: &'a [S]) -> Option<(usize, &'a [S])> {
        if let Some(symbol) = &self.symbol {
            if word.len() >= 1 && &word[0] == symbol {
                return Some((self.next_state, &word[1..]))
            } else {
                return None
            }
        } else {
            return Some((self.next_state, word))
        }
    }
}

pub trait Automaton<S: Symbol> {
    fn new(initial: usize, accepting: Vec<bool>, transitions: Vec<Vec<Transition<S>>>) -> Self where Self: Sized;

    fn initial(&self) -> usize;

    fn accepting(&self, state: usize) -> bool;

    fn transitions(&self, state: usize) -> Vec<Transition<S>>;

    fn accepted_from_state(&self, state: usize, word: &[S]) -> bool {
        word.is_empty() && self.accepting(state) || self.transitions(state)
            .into_iter()
            .any(|transition| {
                if let Some((next_state, suffix)) = transition.next(word) {
                    self.accepted_from_state(next_state, suffix)
                } else {
                    false
                }
            })
    }

    fn accepted(&self, word: &[S]) -> bool {
        self.accepted_from_state(self.initial(), word)
    }
}

pub struct DFA<S: Symbol> {
    size: usize,
    initial: usize,
    accepting: Vec<bool>,
    transitions: Vec<HashMap<S, usize>>,
}

impl<S: Symbol> Automaton<S> for DFA<S> {
    fn new(initial: usize, accepting: Vec<bool>, transitions: Vec<Vec<Transition<S>>>) -> Self {
        let size = accepting.len();
        if transitions.len() != size {
            panic!("size mismatch");
        }
        if initial >= size {
            panic!("initial state index out of bounds");
        }
        for current_transitions in &transitions {
            let mut used_symbols = HashSet::new();
            for transition in current_transitions {
                if transition.next_state >= size {
                    panic!("transition state index out of bounds");
                }
                if let Some(symbol) = &transition.symbol {
                    if used_symbols.contains(symbol) {
                        panic!("multiple transitions with same symbol");
                    }
                    used_symbols.insert(symbol);
                } else {
                    panic!("cannot construct DFA with epsilon transitions");
                }
            }
        }
        let dfa_transitions = transitions
            .into_iter()
            .map(|arr| {
                HashMap::from_iter(arr
                    .into_iter()
                    .map(|transition| {
                        (transition.symbol.expect("should have panicked"), transition.next_state)
                    }))
            })
            .collect();
        DFA { size, initial, accepting, transitions: dfa_transitions }
    }

    fn initial(&self) -> usize {
        self.initial
    }

    fn accepting(&self, state: usize) -> bool {
        self.accepting[state]
    }

    fn transitions(&self, state: usize) -> Vec<Transition<S>> {
        self.transitions[state]
            .iter()
            .map(|(symbol, next_state)| {
                Transition::single_symbol(symbol.clone(), *next_state)
            })
            .collect()
    }

    fn accepted_from_state(&self, state: usize, word: &[S]) -> bool {
        if word.is_empty() {
            self.accepting(state)
        } else {
            self.transitions[state].contains_key(&word[0])
                && self.accepted_from_state(self.transitions[state][&word[0]], &word[1..])
        }
    }
}

pub struct NFA<S: Symbol> {
    size: usize,
    initial: usize,
    accepting: Vec<bool>,
    transitions: Vec<Vec<(Option<S>, usize)>>,
}

impl<S: Symbol> Automaton<S> for NFA<S> {
    fn new(initial: usize, accepting: Vec<bool>, transitions: Vec<Vec<Transition<S>>>) -> Self {
        let size = accepting.len();
        if transitions.len() != size {
            panic!("size mismatch");
        }
        if initial >= size {
            panic!("initial state index out of bounds");
        }
        for current_transitions in &transitions {
            if current_transitions.iter().any(|transition| transition.next_state >= size) {
                panic!("transition state index out of bounds");
            }
        }
        let nfa_transitions = transitions
            .into_iter()
            .map(|arr| {
                arr.into_iter()
                    .map(|transition| {
                        (transition.symbol, transition.next_state)
                    })
                    .collect()
            })
            .collect();
        NFA { size, initial, accepting, transitions: nfa_transitions }
    }

    fn initial(&self) -> usize {
        self.initial
    }

    fn accepting(&self, state: usize) -> bool {
        self.accepting[state]
    }

    fn transitions(&self, state: usize) -> Vec<Transition<S>> {
        self.transitions[state]
            .iter()
            .map(|(symbol_option, next_state)| {
                if let Some(symbol) = symbol_option {
                    Transition::single_symbol(symbol.clone(), *next_state)
                } else {
                    Transition::empty(*next_state)
                }
            })
            .collect()
    }
}

pub struct SingleSymbolNFA<S: Symbol> {
    size: usize,
    initial: usize,
    accepting: Vec<bool>,
    transitions: Vec<HashMap<S, HashSet<usize>>>
}

impl<S: Symbol> Automaton<S> for SingleSymbolNFA<S> {
    fn new(initial: usize, accepting: Vec<bool>, transitions: Vec<Vec<Transition<S>>>) -> Self {
        let size = accepting.len();
        if transitions.len() != size {
            panic!("size mismatch");
        }
        if initial >= size {
            panic!("initial state index out of bounds");
        }
        let mut ss_nfa_transitions = vec![HashMap::<S, HashSet<usize>>::new(); size];
        for (state, current_transitions) in transitions.iter().enumerate() {
            if current_transitions.iter().any(|transition| transition.next_state >= size) {
                panic!("transition state index out of bounds");
            }
            for transition in current_transitions {
                if let Some(symbol) = &transition.symbol {
                    ss_nfa_transitions[state].entry(symbol.clone()).or_default().insert(transition.next_state);
                } else {
                    panic!("cannot construct single-symbol NFA with epsilon transitions");
                }
            }
        }
        SingleSymbolNFA { size, initial, accepting, transitions: ss_nfa_transitions }
    }

    fn initial(&self) -> usize {
        self.initial
    }

    fn accepting(&self, state: usize) -> bool {
        self.accepting[state]
    }

    fn transitions(&self, state: usize) -> Vec<Transition<S>> {
        self.transitions[state]
            .iter()
            .map(|(symbol, next_states)| {
                next_states.iter().map(|next_state| {
                    Transition { next_state: *next_state, symbol: Some(symbol.clone()) }
                })
            })
            .flatten()
            .collect()
    }
}

impl<S: Symbol> SingleSymbolNFA<S> {
    pub fn from_nfa(nfa: &NFA<S>) -> SingleSymbolNFA<S> {
        let (transitions, accepting) = (0..nfa.size)
            .into_iter()
            .map(|state| {
                let mut transitions = HashMap::<S, HashSet<usize>>::new();
                let mut accepting = false;
                let mut queue = VecDeque::<usize>::from([state]);
                while !queue.is_empty() {
                    let current_state = queue.pop_front().expect("queue is not empty");
                    accepting = accepting || nfa.accepting(current_state);
                    for transition in nfa.transitions(current_state) {
                        if let Some(symbol) = transition.symbol {
                            transitions.entry(symbol.clone()).or_default().insert(transition.next_state);
                        } else {
                            queue.push_back(transition.next_state);
                        }
                    }
                }
                (transitions, accepting)
            })
            .unzip();
        SingleSymbolNFA { size: nfa.size, initial: nfa.initial, accepting, transitions }
    }
}

#[cfg(test)]
mod tests;
