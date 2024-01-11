use crate::terminal::candidate_display::{CandidateDisplay, CandidateDisplayState};
use console::Term;
use crate::terminal::voting_display::{VotingDisplay, VotingDisplayState};

pub mod candidate_display;
pub mod voting_display;

pub struct Terminal {
    term: Term,
    state: TerminalState,
}

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            term: Term::buffered_stdout(),
            state: TerminalState::Candidate(CandidateDisplay::new()),
        }
    }

    pub fn handle_input(&mut self) -> anyhow::Result<&TerminalState> {
        match &mut self.state {
            TerminalState::Candidate(candidate_display) => {
                match candidate_display.handle_input(&mut self.term)? {
                    CandidateDisplayState::Done => {
                        self.state = TerminalState::Voting(VotingDisplay::new(
                            candidate_display.candidates().clone(),
                        ));
                    }
                    CandidateDisplayState::Reading => {}
                }
            }
            TerminalState::Voting(voting_display) => {
                match voting_display.handle_input(&mut self.term)? {
                    VotingDisplayState::Voting => {}
                    VotingDisplayState::Done => {
                        self.term.clear_screen()?;
                        self.term.flush()?;

                        self.state = TerminalState::Done;
                    }
                }
            }
            TerminalState::Done => {
                unreachable!()
            }
        }

        Ok(&self.state)
    }
}

#[derive(PartialEq)]
pub enum TerminalState {
    Candidate(CandidateDisplay),
    Voting(VotingDisplay),
    Done,
}
