use super::*;

impl Symbol for char {}

#[test]
fn test_dfa() {
    let automaton = DFA::new(
        0,
        vec![false, true],
        vec![
            HashMap::from([('a', 0), ('b', 1)]),
            HashMap::from([('c', 1)])
        ]
    ).unwrap();
    let accepted_words = ["abc", "b", "aaab", "bc"];
    for word in accepted_words {
        let chars: Vec<char> = word.chars().collect();
        assert!(automaton.accepted(&chars[..]));
    }
    let unaccepted_words = ["ac", "a", "bb", "cba"];
    for word in unaccepted_words {
        let chars: Vec<char> = word.chars().collect();
        assert!(!automaton.accepted(&chars[..]));
    }
}

#[test]
fn test_nfa() {
    let automaton = NFA::new(
        0,
        vec![false, true],
        vec![
            vec![Transition::single_symbol('a', 0), Transition::empty(1)],
            vec![Transition::single_symbol('b', 1)],
        ]
    ).unwrap();
    let accepted_words = ["", "b", "aa", "abbb"];
    for word in accepted_words {
        let chars: Vec<char> = word.chars().collect();
        assert!(automaton.accepted(&chars[..]));
    }
    let unaccepted_words = ["aba", "bba", "c"];
    for word in unaccepted_words {
        let chars: Vec<char> = word.chars().collect();
        assert!(!automaton.accepted(&chars[..]));
    }
}