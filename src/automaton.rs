use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use smallvec::{SmallVec, smallvec};

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
}

#[derive(Debug)]
pub struct Automaton<S: Symbol> {
    alphabet: &'static [S],
    size: usize,
    initial: usize,
    accepting: Vec<bool>,
    transitions: Vec<HashMap<Option<S>, SmallVec<[usize; 1]>>>,
}

impl<S: Symbol> Automaton<S> {
    fn new(alphabet: &'static [S], initial: usize, accepting: Vec<bool>, transitions: Vec<Vec<Transition<S>>>) -> Automaton<S> {
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
        Automaton {
            alphabet,
            size,
            initial,
            accepting,
            transitions: transitions
                .into_iter()
                .map(|arr| {
                    let mut current_transitions: HashMap<Option<S>, SmallVec<[usize; 1]>> = HashMap::new();
                    arr.into_iter()
                        .for_each(|Transition {symbol, next_state}| {
                            current_transitions.entry(symbol).or_default().push(next_state);
                        });
                    current_transitions
                })
                .collect()
        }
    }

    fn alphabet(&self) -> &'static [S] {
        self.alphabet
    }

    fn initial(&self) -> usize {
        self.initial
    }

    fn accepting(&self, state: usize) -> bool {
        self.accepting[state]
    }

    fn symbol_transitions(&self, state: usize, symbol: &S) -> &[usize] {
        let key = Some(symbol.clone());
        if self.transitions[state].contains_key(&key) {
            &self.transitions[state][&key][..]
        } else {
            &[]
        }
    }

    fn empty_transitions(&self, state: usize) -> &[usize] {
        if self.transitions[state].contains_key(&None) {
            &self.transitions[state][&None][..]
        } else {
            &[]
        }
    }

    fn transitions(&self, state: usize) -> Vec<Transition<S>> {
        vec![
            self.alphabet()
                .iter()
                .map(|c| {
                    self.symbol_transitions(state, c).into_iter().map(|next_state| {
                        Transition::single_symbol(c.clone(), *next_state)
                    }).collect::<Vec<_>>()
                })
                .flatten()
                .collect::<Vec<_>>(),
            self.empty_transitions(state).into_iter().map(|next_state| {
                    Transition::empty(*next_state)
                })
                .collect()
        ].concat()
    }

    fn reached_by_epsilon(&self, state: usize) -> SmallVec<[usize; 1]> {
        if self.empty_transitions(state).is_empty() {
            smallvec![state]
        } else {
            let mut visited: HashSet<usize> = HashSet::from([state]);
            let mut queue: VecDeque<usize> = VecDeque::from([state]);
            loop {
                if let Some(current_state) = queue.pop_front() {
                    self.empty_transitions(current_state)
                        .iter()
                        .for_each(|next_state| {
                            if !visited.contains(next_state) {
                                visited.insert(*next_state);
                                queue.push_back(*next_state);
                            }
                        })
                } else {
                    break;
                }
            }
            visited.into_iter().collect()
        }
    }

    fn accepted_from_state(&self, state: usize, word: &[S]) -> bool {
        if word.is_empty() {
            self.reached_by_epsilon(state).into_iter().any(|reached_state| {
                self.accepting(reached_state)
            })
        } else {
            self.reached_by_epsilon(state).into_iter().any(|reached_state| {
                self.symbol_transitions(reached_state, &word[0]).iter().any(|next_state| {
                    self.accepted_from_state(*next_state, &word[1..])
                })
            })
        }
    }

    fn accepted(&self, word: &[S]) -> bool {
        self.accepted_from_state(self.initial(), word)
    }

    fn is_single_symbol(&self) -> bool {
        (0..self.size)
            .into_iter()
            .all(|state| self.empty_transitions(state).is_empty())
    }

    fn is_dfa(&self) -> bool {
        self.is_single_symbol() &&
            (0..self.size)
            .into_iter()
            .all(|state| {
                self.alphabet().iter().all(|c| self.symbol_transitions(state, c).len() <= 1)
            })
    }

    fn single_symbol_nfa_from(automaton: &Automaton<S>) -> Automaton<S> {
        let accepting = (0..automaton.size)
            .into_iter()
            .map(|state| {
                automaton.reached_by_epsilon(state)
                    .into_iter()
                    .any(|reached_state| {
                        automaton.accepting(reached_state)
                    })
            })
            .collect();
        let transitions = (0..automaton.size)
            .into_iter()
            .map(|state| {
                automaton.reached_by_epsilon(state)
                    .into_iter()
                    .map(|reached_state| {
                        automaton.transitions(reached_state)
                            .into_iter()
                            .filter(|transition| {
                                transition.symbol.is_some()
                            })
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .collect();
        Automaton::new(automaton.alphabet, automaton.initial, accepting, transitions)
    }

    fn dfa_from(automaton: &Automaton<S>) -> Automaton<S> {
        let ss_nfa = Automaton::single_symbol_nfa_from(automaton);
        let mut visited_masks = HashMap::new();
        let mut transitions = vec![];
        let mut accepting = vec![];
        let mut queue = VecDeque::new();
        let initial_mask: Vec<_> = (0..ss_nfa.size).map(|state| state == ss_nfa.initial).collect();
        visited_masks.insert(initial_mask.clone(), 0);
        queue.push_back(initial_mask.clone());
        while !queue.is_empty() {
            let mask = queue.pop_front().expect("queue is not empty");
            let states = mask.clone()
                .into_iter()
                .enumerate()
                .filter(|(_, has)| *has)
                .unzip::<usize, bool, Vec<_>, Vec<_>>().0;
            accepting.push(states.iter().any(|state| ss_nfa.accepting(*state)));
            transitions.push(vec![]);
            for c in ss_nfa.alphabet {
                let mut next_mask = vec![false; ss_nfa.size];
                states
                    .iter()
                    .for_each(|state| {
                        ss_nfa.symbol_transitions(*state, c)
                            .iter()
                            .for_each(|next_state| {
                                next_mask[*next_state] = true;
                            });
                    });
                if !next_mask.iter().any(|x| *x) {
                    continue;
                }
                if !visited_masks.contains_key(&next_mask) {
                    visited_masks.insert(next_mask.clone(), visited_masks.len());
                    queue.push_back(next_mask.clone());
                }
                transitions[visited_masks[&mask]].push(Transition::single_symbol(c.clone(), visited_masks[&next_mask]));
            }
        }
        Automaton::new(automaton.alphabet, automaton.initial, accepting, transitions)
    }
}

#[cfg(test)]
mod tests;
