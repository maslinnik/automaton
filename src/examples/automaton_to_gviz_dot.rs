fn main() {
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
    println!("NFA GraphViz:\n{}", automaton_to_gviz_dot(&nfa));
    println!("DFA GraphViz:\n{}", automaton_to_gviz_dot(&dfa));
}