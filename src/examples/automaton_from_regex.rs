fn main() {
    let automaton = Automaton::from_regex(&['a', 'b'], &parse_regex_from_string("a*b|ababa"));
    assert!(accepted_str(&automaton, "ababa"));
    assert!(accepted_str(&automaton, "aaaab"));
    assert!(accepted_str(&automaton, "b"));
    assert!(!accepted_str(&automaton, "baba"));
    assert!(!accepted_str(&automaton, "baaa"));
    assert!(!accepted_str(&automaton, "aabb"));
    println!("MCDFA GraphViz:\n{}", automaton_to_gviz_dot(&Automaton::minimal_complete_dfa_from(&automaton)));
}
