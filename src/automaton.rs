use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;
use std::hash::Hash;
use smallvec::{SmallVec, smallvec};

pub trait Symbol: Eq + Clone + Hash + 'static {}

#[derive(Clone)]
pub enum Regex<S: Symbol> {
    String(Vec<S>),
    Concat(Box<Regex<S>>, Box<Regex<S>>),
    Union(Box<Regex<S>>, Box<Regex<S>>),
    KleeneStar(Box<Regex<S>>)
}

impl<S: Symbol> Regex<S> {
    fn concat(lhs: Regex<S>, rhs: Regex<S>) -> Regex<S> {
        use Regex::*;
        if let String(lhs_vec) = &lhs {
            if lhs_vec.is_empty() {
                return rhs;
            }
        }
        if let String(rhs_vec) = &rhs {
            if rhs_vec.is_empty() {
                return lhs;
            }
        }
        if let String(lhs_vec) = &lhs {
            if let String(rhs_vec) = &rhs {
                return String(vec![lhs_vec.clone(), rhs_vec.clone()].concat());
            }
        }
        Concat(Box::new(lhs), Box::new(rhs))
    }

    fn union(lhs: Regex<S>, rhs: Regex<S>) -> Regex<S> {
        Regex::Union(Box::new(lhs), Box::new(rhs))
    }

    fn kleene_star(regex: Regex<S>) -> Regex<S> {
        use Regex::*;
        if let String(vec) = &regex {
            if vec.is_empty() {
                return String(vec![]);
            }
        }
        KleeneStar(Box::new(regex))
    }
}

impl<S: Symbol> Default for Regex<S> {
    fn default() -> Self {
        Regex::String(vec![])
    }
}

impl<S: Symbol + fmt::Display> fmt::Display for Regex<S> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Regex::String(vec) => {
                for c in vec {
                    f.write_fmt(format_args!("({})", c))?;
                }
            }
            Regex::Concat(lhs, rhs) => {
                f.write_fmt(format_args!("({}{})", lhs, rhs))?;
            }
            Regex::Union(lhs, rhs) => {
                f.write_fmt(format_args!("({}|{})", lhs, rhs))?;
            }
            Regex::KleeneStar(regex) => {
                f.write_fmt(format_args!("({}*)", regex))?;
            }
        }
        Ok(())
    }
}

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

#[derive(Debug, Clone)]
pub struct Automaton<S: Symbol> {
    alphabet: &'static [S],
    size: usize,
    initial: usize,
    accepting: Vec<bool>,
    transitions: Vec<HashMap<Option<S>, SmallVec<[usize; 1]>>>,
}

impl<S: Symbol> Automaton<S> {
    fn new(alphabet: &'static [S], size: usize) -> Automaton<S> {
        Automaton::from(alphabet, 0, vec![false; size], vec![vec![]; size])
    }

    fn from(alphabet: &'static [S], initial: usize, accepting: Vec<bool>, transitions: Vec<Vec<Transition<S>>>) -> Automaton<S> {
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

    fn size(&self) -> usize {
        self.size
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

    fn set_size(&mut self, new_size: usize) {
        if new_size < self.size {
            panic!("cannot set smaller size");
        }
        self.size = new_size;
        self.accepting.resize(new_size, false);
        self.transitions.resize(new_size, HashMap::default());
    }

    fn set_initial(&mut self, new_initial: usize) {
        if new_initial >= self.size {
            panic!("new initial state index out of bounds");
        }
        self.initial = new_initial;
    }

    fn set_accepting(&mut self, state: usize, new_accepting: bool) {
        if state >= self.size {
            panic!("state index out of bounds");
        }
        self.accepting[state] = new_accepting;
    }

    fn add_empty_transition(&mut self, from: usize, to: usize) {
        if from >= self.size || to >= self.size {
            panic!("transition state index out of bounds");
        }
        self.transitions[from].entry(None).or_default().push(to);
    }

    fn add_symbol_transition(&mut self, from: usize, to: usize, symbol: S) {
        if from >= self.size || to >= self.size {
            panic!("transition state index out of bounds");
        }
        self.transitions[from].entry(Some(symbol)).or_default().push(to);
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

    fn reached(&self, state: usize) -> Vec<usize> {
        let mut visited: HashSet<usize> = HashSet::from([state]);
        let mut queue: VecDeque<usize> = VecDeque::from([state]);
        loop {
            if let Some(current_state) = queue.pop_front() {
                self.transitions(current_state)
                    .iter()
                    .for_each(|Transition { next_state, .. }| {
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
        (0..self.size())
            .into_iter()
            .all(|state| self.empty_transitions(state).is_empty())
    }

    fn is_dfa(&self) -> bool {
        self.is_single_symbol() &&
            (0..self.size())
            .into_iter()
            .all(|state| {
                self.alphabet().iter().all(|c| self.symbol_transitions(state, c).len() <= 1)
            })
    }

    fn is_complete_dfa(&self) -> bool {
        self.is_single_symbol() &&
            (0..self.size())
                .into_iter()
                .all(|state| {
                    self.alphabet().iter().all(|c| self.symbol_transitions(state, c).len() == 1)
                })
    }

    fn single_symbol_nfa_from(automaton: &Automaton<S>) -> Automaton<S> {
        if automaton.is_single_symbol() {
            return automaton.clone();
        }
        let accepting = (0..automaton.size())
            .into_iter()
            .map(|state| {
                automaton.reached_by_epsilon(state)
                    .into_iter()
                    .any(|reached_state| {
                        automaton.accepting(reached_state)
                    })
            })
            .collect();
        let transitions = (0..automaton.size())
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
        Automaton::from(automaton.alphabet(), automaton.initial(), accepting, transitions)
    }

    fn dfa_from(automaton: &Automaton<S>) -> Automaton<S> {
        if automaton.is_dfa() {
            return automaton.clone();
        }
        let ss_nfa = Automaton::single_symbol_nfa_from(automaton);
        let mut visited_masks = HashMap::new();
        let mut transitions = vec![];
        let mut accepting = vec![];
        let mut queue = VecDeque::new();
        let initial_mask: Vec<_> = (0..ss_nfa.size()).map(|state| state == ss_nfa.initial()).collect();
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
            for c in ss_nfa.alphabet() {
                let mut next_mask = vec![false; ss_nfa.size()];
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
        Automaton::from(automaton.alphabet(), automaton.initial(), accepting, transitions)
    }

    fn complete_dfa_from(automaton: &Automaton<S>) -> Automaton<S> {
        if automaton.is_complete_dfa() {
            return automaton.clone();
        }
        let mut dfa = Automaton::dfa_from(automaton);
        if !dfa.is_complete_dfa() {
            dfa.set_size(dfa.size() + 1);
            let halting_state = dfa.size() - 1;
            for state in 0..dfa.size() {
                for c in dfa.alphabet() {
                    if dfa.symbol_transitions(state, c).is_empty() {
                        dfa.add_symbol_transition(state, halting_state, c.clone());
                    }
                }
            }
        }
        dfa
    }

    fn minimal_complete_dfa_from(automaton: &Automaton<S>) -> Automaton<S> {
        let cdfa = Automaton::complete_dfa_from(automaton);
        let reached_from_initial = cdfa.reached(cdfa.initial());
        let mut classes = vec![
            HashSet::from_iter(reached_from_initial.clone().into_iter().filter(|state| cdfa.accepting(*state))),
            HashSet::from_iter(reached_from_initial.clone().into_iter().filter(|state| !cdfa.accepting(*state)))
        ];
        for _len in 1..cdfa.size() {
            for c in cdfa.alphabet() {
                classes = classes.into_iter()
                    .map(|class| {
                        let mut split_classes: HashMap<usize, HashSet<usize>> = HashMap::new();
                        class
                            .into_iter()
                            .for_each(|state| {
                                split_classes.entry(cdfa.symbol_transitions(state, c)[0]).or_default().insert(state);
                            });
                        split_classes.into_values().collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect();
            }
        }
        classes.sort_by_cached_key(|class| class.clone().into_iter().min().expect("all classes are non-empty"));
        let mut class_index = vec![0; cdfa.size()];
        classes.iter()
            .enumerate()
            .for_each(|(index, class)| {
                class.iter()
                    .for_each(|state| { class_index[*state] = index; });
            });
        let accepting = classes.iter()
            .map(|class| {
                class.iter()
                    .any(|state| cdfa.accepting(*state))
            })
            .collect();
        let transitions = classes.iter()
            .map(|class| {
                let state = *class.iter().next().expect("all classes are non-empty");
                cdfa.transitions(state).into_iter()
                    .map(|Transition { next_state, symbol }| {
                        Transition { next_state: class_index[next_state], symbol }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Automaton::from(
            cdfa.alphabet(),
            class_index[cdfa.initial()],
            accepting,
            transitions
        )
    }

    fn regex(&self) -> Regex<S> {
        use Regex::*;
        let mut regex_transitions = (0..self.size())
            .into_iter()
            .map(|state| {
                let mut transitions: HashMap<usize, Regex<S>> = HashMap::new();
                self.transitions(state)
                    .into_iter()
                    .for_each(|transition| {
                        let current_regex = if let Some(symbol) = transition.symbol { String(vec![symbol]) } else { String(vec![]) };
                        if transitions.contains_key(&transition.next_state) {
                            let edge_regex = transitions[&transition.next_state].clone();
                            transitions.insert(transition.next_state, Regex::union(edge_regex, current_regex));
                        } else {
                            transitions.insert(transition.next_state, current_regex);
                        }
                    });
                if self.accepting(state) {
                    transitions.insert(self.size(), String(vec![]));
                }
                transitions
            })
            .collect::<Vec<_>>();
        for state in 0..self.size() {
            if state == self.initial() {
                continue;
            }
            let transitions = regex_transitions[state].clone();
            for from in 0..self.size() {
                if from == state {
                    continue;
                }
                if regex_transitions[from].contains_key(&state) {
                    for (to, regex) in &transitions {
                        if *to == state {
                            continue;
                        }
                        let current_regex = Regex::concat(
                            regex_transitions[from][&state].clone(),
                            if regex_transitions[state].contains_key(&state) {
                                Regex::concat(Regex::kleene_star(regex_transitions[state][&state].clone()), regex.clone())
                            } else {
                                regex.clone()
                            }
                        );
                        if regex_transitions[from].contains_key(to) {
                            let edge_regex = regex_transitions[from][to].clone();
                            regex_transitions[from].insert(*to, Regex::union(edge_regex, current_regex));
                        } else {
                            regex_transitions[from].insert(*to, current_regex);
                        }
                    }
                    regex_transitions[from].remove(&state);
                }
            }
        }
        if !regex_transitions[self.initial()].contains_key(&self.size()) {
            panic!("cannot construct regex from automaton that accepts no words");
        }
        if regex_transitions[self.initial()].contains_key(&self.initial()) {
            Regex::concat(Regex::kleene_star(regex_transitions[self.initial()][&self.initial].clone()),
                          regex_transitions[self.initial()][&self.size()].clone())
        } else {
            regex_transitions[self.initial()][&self.size()].clone()
        }
    }
}

#[cfg(test)]
mod tests;
