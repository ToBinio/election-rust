use crate::terminal::{Terminal, TerminalState};

mod terminal;
mod utils;

fn main() {
    let mut terminal = Terminal::new();

    while terminal.handle_input().unwrap() != &TerminalState::Done {}
}
