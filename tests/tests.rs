use automaton::*;
use automaton::char_automaton::*;

#[test]
fn test_dfa() {
    let mut automaton = Automaton::new(&['a', 'b', 'c'], 2);
    automaton.set_initial(0);
    automaton.add_symbol_transition(0, 0, 'a');
    automaton.add_symbol_transition(0, 1, 'b');
    automaton.add_symbol_transition(1, 1, 'c');
    automaton.set_accepting(1, true);
    assert!(automaton.is_dfa());
    let accepted_words = ["abc", "b", "aaab", "bc"];
    for word in accepted_words {
        assert!(accepted_str(&automaton, word));
    }
    let unaccepted_words = ["ac", "a", "bb", "cba"];
    for word in unaccepted_words {
        assert!(!accepted_str(&automaton, word));
    }
}

#[test]
fn test_nfa() {
    let mut automaton = Automaton::new(&['a', 'b'], 2);
    automaton.set_initial(0);
    automaton.add_symbol_transition(0, 0, 'a');
    automaton.add_empty_transition(0, 1);
    automaton.add_symbol_transition(1, 1, 'b');
    automaton.set_accepting(1, true);
    let accepted_words = ["", "b", "aa", "abbb"];
    for word in accepted_words {
        assert!(accepted_str(&automaton, word));
    }
    let unaccepted_words = ["aba", "bba", "c"];
    for word in unaccepted_words {
        assert!(!accepted_str(&automaton, word));
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Binary {
    Zero,
    One
}

#[test]
fn test_nfa_with_enum() {
    use Binary::*;
    let mut automaton = Automaton::new(&[Zero, One], 2);
    automaton.set_initial(0);
    automaton.add_symbol_transition(0, 0, Zero);
    automaton.add_empty_transition(0, 1);
    automaton.add_symbol_transition(1, 1, One);
    automaton.set_accepting(1, true);
    let accepted_words = [vec![], vec![One], vec![Zero, Zero], vec![Zero, One, One, One]];
    for word in accepted_words {
        assert!(automaton.accepted(&word[..]));
    }
    let unaccepted_words = [vec![One, Zero, One], vec![Zero, One, Zero]];
    for word in unaccepted_words {
        assert!(!automaton.accepted(&word[..]));
    }
}

fn stress_automaton_equivalence(one: &Automaton<char>, two: &Automaton<char>, max_len: usize) {
    assert_eq!(one.alphabet(), two.alphabet());
    assert_eq!(one.accepted(&[]), two.accepted(&[]));
    let mut current_words: Vec<Vec<char>> = vec![vec![]];
    for _ in 1..=max_len {
        current_words = current_words
            .into_iter()
            .map(|word| {
                one.alphabet().iter().map(move |c| {
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
fn test_nfa_to_ss_nfa() {
    let nfa = Automaton::from(
        &['a', 'b'],
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
    let ss_nfa = Automaton::single_symbol_nfa_from(&nfa);
    assert!(ss_nfa.is_single_symbol());
    stress_automaton_equivalence(&nfa, &ss_nfa, 12);
}

#[test]
fn test_nfa_to_dfa() {
    let nfa = Automaton::from(
        &['0', '1'],
        0,
        vec![false, false, false, false, true],
        vec![
            vec![Transition::single_symbol('0', 0), Transition::single_symbol('1', 0), Transition::single_symbol('1', 1)],
            vec![Transition::single_symbol('0', 2), Transition::single_symbol('1', 2)],
            vec![Transition::single_symbol('0', 3), Transition::single_symbol('1', 3)],
            vec![Transition::single_symbol('0', 4), Transition::single_symbol('1', 4)],
            vec![]
        ]
    );
    let dfa = Automaton::dfa_from(&nfa);
    assert!(dfa.is_dfa());
    stress_automaton_equivalence(&nfa, &dfa, 10);
}

#[test]
fn test_nfa_to_complete_dfa() {
    let nfa = Automaton::from(
        &['0', '1'],
        0,
        vec![false, false, false, false, true],
        vec![
            vec![Transition::single_symbol('0', 0), Transition::single_symbol('1', 0), Transition::single_symbol('1', 1)],
            vec![Transition::single_symbol('0', 2), Transition::single_symbol('1', 2)],
            vec![Transition::single_symbol('0', 3), Transition::single_symbol('1', 3)],
            vec![Transition::single_symbol('0', 4), Transition::single_symbol('1', 4)],
            vec![]
        ]
    );
    let complete_dfa = Automaton::complete_dfa_from(&nfa);
    assert!(complete_dfa.is_complete_dfa());
    stress_automaton_equivalence(&nfa, &complete_dfa, 10);
}

#[test]
fn test_dfa_to_minimal_complete_dfa() {
    let mut dfa = Automaton::new(&['a', 'b'], 4);
    dfa.add_symbol_transition(0, 3, 'a');
    dfa.add_symbol_transition(0, 1, 'b');
    dfa.add_symbol_transition(1, 1, 'a');
    dfa.add_symbol_transition(1, 2, 'b');
    dfa.add_symbol_transition(2, 3, 'a');
    dfa.add_symbol_transition(2, 1, 'b');
    dfa.add_symbol_transition(3, 3, 'a');
    dfa.add_symbol_transition(3, 3, 'b');
    dfa.set_initial(0);
    dfa.set_accepting(3, true);
    assert!(dfa.is_complete_dfa());
    let mcdfa = Automaton::minimal_complete_dfa_from(&dfa);
    assert!(mcdfa.is_complete_dfa());
    assert_eq!(mcdfa.size(), 3);
    stress_automaton_equivalence(&dfa, &mcdfa, 10);
}

#[test]
fn test_to_regex_and_back() {
    let mut automaton = Automaton::new(&['a', 'b'], 4);
    automaton.add_symbol_transition(0, 3, 'a');
    automaton.add_symbol_transition(0, 1, 'b');
    automaton.add_symbol_transition(1, 1, 'a');
    automaton.add_symbol_transition(1, 2, 'b');
    automaton.add_symbol_transition(2, 3, 'a');
    automaton.add_symbol_transition(2, 1, 'b');
    automaton.add_symbol_transition(3, 3, 'a');
    automaton.add_symbol_transition(3, 3, 'b');
    automaton.set_initial(0);
    automaton.set_accepting(3, true);
    let regex = automaton.regex();
    let automaton_from_regex = Automaton::dfa_from(&Automaton::from_regex(automaton.alphabet(), &regex));
    stress_automaton_equivalence(&automaton, &automaton_from_regex, 15);
}

#[test]
fn test_to_string_and_back() {
    let mut automaton = Automaton::new(&['a', 'b'], 2);
    automaton.set_initial(0);
    automaton.add_symbol_transition(0, 0, 'a');
    automaton.add_empty_transition(0, 1);
    automaton.add_symbol_transition(1, 1, 'b');
    automaton.set_accepting(1, true);
    let string = automaton_to_string(&automaton);
    let automaton_from_string = automaton_from_string(&['a', 'b'], &string);
    stress_automaton_equivalence(&automaton, &automaton_from_string, 15);
}

#[test]
fn test_to_string_regex_and_back() {
    let mut automaton = Automaton::new(&['a', 'b'], 4);
    automaton.add_symbol_transition(0, 3, 'a');
    automaton.add_symbol_transition(0, 1, 'b');
    automaton.add_symbol_transition(1, 1, 'a');
    automaton.add_symbol_transition(1, 2, 'b');
    automaton.add_symbol_transition(2, 3, 'a');
    automaton.add_symbol_transition(2, 1, 'b');
    automaton.add_symbol_transition(3, 3, 'a');
    automaton.add_symbol_transition(3, 3, 'b');
    automaton.set_initial(0);
    automaton.set_accepting(3, true);
    let regex = automaton.regex();
    let regex_string = format!("{}", regex);
    let regex_from_string = parse_regex_from_string(&regex_string);
    let automaton_from_regex = Automaton::dfa_from(&Automaton::from_regex(automaton.alphabet(), &regex_from_string));
    stress_automaton_equivalence(&automaton, &automaton_from_regex, 15);
}