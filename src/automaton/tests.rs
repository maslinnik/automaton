use super::*;

impl Symbol for char {}

#[test]
fn test_dfa() {
    let automaton = DFA::new(
        0,
        vec![false, true],
        vec![
            vec![Transition::single_symbol('a', 0), Transition::single_symbol('b', 1)],
            vec![Transition::single_symbol('c', 1)]
        ]
    );
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
            vec![Transition::single_symbol('b', 1)]
        ]
    );
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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Binary {
    Zero,
    One
}

impl Symbol for Binary {}

#[test]
fn test_nfa_with_enum() {
    use Binary::*;
    let automaton = NFA::new(
        0,
        vec![false, true],
        vec![
            vec![Transition::single_symbol(Zero, 0), Transition::empty(1)],
            vec![Transition::single_symbol(One, 1)]
        ]
    );
    let accepted_words = [vec![], vec![One], vec![Zero, Zero], vec![Zero, One, One, One]];
    for word in accepted_words {
        assert!(automaton.accepted(&word[..]));
    }
    let unaccepted_words = [vec![One, Zero, One], vec![Zero, One, Zero]];
    for word in unaccepted_words {
        assert!(!automaton.accepted(&word[..]));
    }
}

fn stress_automaton_equivalence(one: &dyn Automaton<char>, two: &dyn Automaton<char>, chars: Vec<char>, max_len: usize) {
    assert_eq!(one.accepted(&[]), two.accepted(&[]));
    let mut current_words: Vec<Vec<char>> = vec![vec![]];
    for _ in 1..=max_len {
        current_words = current_words
            .into_iter()
            .map(|word| {
                chars.iter().map(move |c| {
                    vec![word.clone(), vec![c.clone()]].concat()
                })
            })
            .flatten()
            .collect();
        for word in &current_words {
            assert_eq!(one.accepted(&word[..]), two.accepted(&word[..]));
        }
    }
}

#[test]
fn test_ss_nfa() {
    let nfa = NFA::new(
        0,
        vec![false, true, false, false, false, false],
        vec![
            vec![Transition::single_symbol('a', 1)],
            vec![Transition::empty(2)],
            vec![Transition::single_symbol('b', 3), Transition::single_symbol('a', 4)],
            vec![Transition::single_symbol('a', 2)],
            vec![Transition::single_symbol('a', 5), Transition::empty(1)],
            vec![Transition::single_symbol('b', 4)]
        ]
    );
    let ss_nfa = SingleSymbolNFA::from_nfa(&nfa);
    stress_automaton_equivalence(&nfa, &ss_nfa, vec!['a', 'b'], 12);
}
