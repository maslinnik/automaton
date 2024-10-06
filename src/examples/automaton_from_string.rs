fn main() {
    let automaton = automaton_from_string(
        &['a', 'b'],
        "0\n1\n0 0 a\n0 1\n1 1 b"
    );
    assert!(accepted_str(&automaton, "aabb"));
    assert!(!accepted_str(&automaton, "aba"));
}