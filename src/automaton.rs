use std::collections::HashMap;

pub trait Symbol: PartialEq + Clone + 'static {}

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

impl<S: Symbol> DFA<S> {
    pub fn new(initial: usize, accepting: Vec<bool>, transitions: Vec<HashMap<S, usize>>) -> Result<DFA<S>, &'static str> {
        let size = accepting.len();
        if transitions.len() != size {
            return Err("size mismatch");
        }
        if initial >= size {
            return Err("initial state index out of bounds");
        }
        for current_transitions in &transitions {
            if current_transitions.iter().any(|(_, next_state)| *next_state >= size) {
                return Err("transition state index out of bounds");
            }
        }
        Ok(DFA { size, initial, accepting, transitions })
    }
}

impl<S: Symbol> Automaton<S> for DFA<S> {
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
    transitions: Vec<Vec<Transition<S>>>,
}

impl<S: Symbol> NFA<S> {
    pub fn new(initial: usize, accepting: Vec<bool>, transitions: Vec<Vec<Transition<S>>>) -> Result<NFA<S>, &'static str> {
        let size = accepting.len();
        if transitions.len() != size {
            return Err("size mismatch");
        }
        if initial >= size {
            return Err("initial state index out of bounds");
        }
        for current_transitions in &transitions {
            if current_transitions.iter().any(|transition| transition.next_state >= size) {
                return Err("transition state index out of bounds");
            }
        }
        Ok(NFA { size, initial, accepting, transitions })
    }
}

impl<S: Symbol> Automaton<S> for NFA<S> {
    fn initial(&self) -> usize {
        self.initial
    }

    fn accepting(&self, state: usize) -> bool {
        self.accepting[state]
    }

    fn transitions(&self, state: usize) -> impl Iterator<Item=Transition<S>> {
        self.transitions[state].clone().into_iter()
    }
}

#[cfg(test)]
mod tests;
