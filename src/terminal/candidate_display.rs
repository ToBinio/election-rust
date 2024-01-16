use crate::utils::candidate::{save_candidates, FILE_PATH};
use console::Term;
use std::io::Write;

pub struct CandidateDisplay {
    term: Term,
    candidates: Vec<String>,
}

impl CandidateDisplay {
    pub fn new() -> CandidateDisplay {
        CandidateDisplay {
            term: Term::buffered_stdout(),
            candidates: vec![],
        }
    }

    pub fn handle_input(&mut self) -> anyhow::Result<CandidateDisplayState> {
        write!(self.term, "> ")?;
        self.term.flush()?;

        let name = self.term.read_line()?;

        return match name.as_str() {
            "save" => {
                save_candidates(FILE_PATH, &self.candidates)?;
                Ok(CandidateDisplayState::Done)
            }
            _ => {
                self.candidates.push(name.to_string());
                Ok(CandidateDisplayState::Reading)
            }
        };
    }
}

#[derive(PartialEq)]
pub enum CandidateDisplayState {
    Reading,
    Done,
}
