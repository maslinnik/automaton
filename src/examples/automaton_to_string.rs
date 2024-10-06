use automaton::*;
use automaton::char_automaton::*;

fn main() {
    let mut automaton = Automaton::new(&['α', 'β', 'γ'], 3);
    automaton.set_initial(0);
    automaton.set_accepting(2, true);
    automaton.add_symbol_transition(0, 1, 'α');
    automaton.add_symbol_transition(1, 1, 'β');
    automaton.add_symbol_transition(1, 2, 'γ');
    println!("String representation:\n{}", automaton_to_string(&automaton));
}