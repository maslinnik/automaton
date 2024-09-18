mod automaton;

use crate::automaton::{Symbol, Automaton, DFA};
use std::collections::HashMap;

impl Symbol for char {}

fn main() {
    let automaton = DFA::new(
        0,
        vec![false, true],
        vec![
            HashMap::from([('a', 0), ('b', 1)]),
            HashMap::from([('c', 1)])
        ]
    ).unwrap();
    let words = ["abc", "b", "aaab", "bc", "ac", "a", "bb", "cba"];
    for word in words {
        let chars: Vec<char> = word.chars().collect();
        let accepted = automaton.accepted(&chars[..]);
        println!("{} {}", word, accepted);
    }
}
