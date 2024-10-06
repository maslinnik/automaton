use super::*;
use std::cmp::max;

pub fn automaton_from_string(alphabet: &'static [char], string: &str) -> Automaton<char> {
    let mut automaton = Automaton::new(alphabet, 1);
    let mut lines = string.lines();
    let initial: usize = lines.next().unwrap().parse().unwrap();
    automaton.set_size(max(automaton.size(), initial + 1));
    automaton.set_initial(initial);
    let accepting: Vec<usize> = lines.next().unwrap().split(' ').map(|token| {
        token.parse().unwrap()
    }).collect();
    for accepting_state in accepting {
        automaton.set_size(max(automaton.size(), accepting_state + 1));
        automaton.set_accepting(accepting_state, true);
    }
    for line in lines {
        let tokens = line.split(' ').collect::<Vec<_>>();
        let from: usize = tokens[0].parse().unwrap();
        let to: usize = tokens[1].parse().unwrap();
        automaton.set_size(max(automaton.size(), from + 1));
        automaton.set_size(max(automaton.size(), to + 1));
        if tokens.len() == 2 {
            automaton.add_empty_transition(from, to);
        } else {
            assert_eq!(tokens[2].len(), 1);
            let c = &tokens[2].chars().next().unwrap();
            assert!(alphabet.contains(c));
            automaton.add_symbol_transition(from, to, c.clone());
        }
    }
    automaton
}

pub fn automaton_to_string(automaton: &Automaton<char>) -> String {
    let mut result = String::new();
    result += &format!("{}\n", automaton.initial());
    result += &format!("{}\n", (0..automaton.size()).into_iter()
        .filter(|state| automaton.accepting(*state))
        .map(|state| state.to_string())
        .collect::<Vec<_>>()
        .join(" "));
    for state in 0..automaton.size() {
        for transition in automaton.transitions(state) {
            if let Some(c) = transition.symbol {
                result += &format!("{} {} {}\n", state, transition.next_state, c);
            } else {
                result += &format!("{} {}\n", state, transition.next_state);
            }
        }
    }
    result
}

pub fn automaton_to_gviz_dot(automaton: &Automaton<char>) -> String {
    let mut result = String::new();
    result += "digraph {\n";
    result += "phantom [label=\"\", shape=none, height=0, width=0]\n";
    result += &format!("phantom -> {}\n", automaton.initial());
    for state in 0..automaton.size() {
        if automaton.accepting(state) {
            result += &format!("{} [shape=doublecircle]\n", state);
        } else {
            result += &format!("{} [shape=circle]\n", state);
        }
    }
    for state in 0..automaton.size() {
        for transition in automaton.transitions(state) {
            if let Some(c) = transition.symbol {
                result += &format!("{} -> {} [label=\"{}\"]\n", state, transition.next_state, c);
            } else {
                result += &format!("{} -> {} [label=\"ε\"]\n", state, transition.next_state);
            }
        }
    }
    result += "}\n";
    result
}

pub fn accepted_str(automaton: &Automaton<char>, word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    automaton.accepted(&chars[..])
}