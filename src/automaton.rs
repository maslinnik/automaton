use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait Symbol: Eq + Clone + Hash + 'static {}

#[derive(Clone)]
pub struct Transition<S: Symbol> {
    next_state: usize,
    symbols: Vec<S>,
}

impl<S: Symbol> Transition<S> {
    fn empty(next_state: usize) -> Transition<S> {
        Transition { next_state, symbols: vec![] }
    }

    fn single_symbol(symbol: S, next_state: usize) -> Transition<S> {
        Transition { next_state, symbols: vec![symbol] }
    }

    fn multiple_symbols(symbols: Vec<S>, next_state: usize) -> Transition<S> {
        Transition { next_state, symbols }
    }

    fn next<'a>(&self, word: &'a [S]) -> Option<(usize, &'a [S])> {
        let len = self.symbols.len();
        if word.len() >= len && &word[..len] == self.symbols {
            Some((self.next_state, &word[len..]))
        } else {
            None
        }
    }
}

pub trait Automaton<S: Symbol> {
    fn new(initial: usize, accepting: Vec<bool>, transitions: Vec<Vec<Transition<S>>>) -> Self;

    fn initial(&self) -> usize;

    fn accepting(&self, state: usize) -> bool;

    fn transitions(&self, state: usize) -> impl Iterator<Item=Transition<S>>;

    fn accepted_from_state(&self, state: usize, word: &[S]) -> bool {
        word.is_empty() && self.accepting(state) || self.transitions(state)
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
                if transition.symbols.len() != 1 {
                    panic!("cannot construct DFA with non-single-symbol transitions");
                }
                if used_symbols.contains(&transition.symbols[0]) {
                    panic!("multiple transitions with same symbol");
                }
                used_symbols.insert(&transition.symbols[0]);
            }
            if current_transitions.iter().any(|transition| transition.next_state >= size) {
                panic!("transition state index out of bounds");
            }
        }
        let dfa_transitions = transitions
            .into_iter()
            .map(|arr| {
                HashMap::from_iter(arr
                    .into_iter()
                    .map(|transition| {
                        (transition.symbols[0].clone(), transition.next_state)
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

    fn transitions(&self, state: usize) -> impl Iterator<Item=Transition<S>> {
        self.transitions[state]
            .iter()
            .map(|(symbol, next_state)| {
                Transition::single_symbol(symbol.clone(), *next_state)
            })
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
            if current_transitions.iter().any(|transition| transition.symbols.len() > 1) {
                panic!("cannot construct NFA with multi-symbol transitions");
            }
        }
        let nfa_transitions = transitions
            .into_iter()
            .map(|arr| {
                arr.into_iter()
                    .map(|transition| {
                        if transition.symbols.len() == 0 {
                            (None, transition.next_state)
                        } else {
                            (Some(transition.symbols[0].clone()), transition.next_state)
                        }
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

    fn transitions(&self, state: usize) -> impl Iterator<Item=Transition<S>> {
        self.transitions[state]
            .iter()
            .map(|(symbol_option, next_state)| {
                if let Some(symbol) = symbol_option {
                    Transition::single_symbol(symbol.clone(), *next_state)
                } else {
                    Transition::empty(*next_state)
                }
            })
    }
}

#[cfg(test)]
mod tests;
