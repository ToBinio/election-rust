use console::{style, Term};
use std::io::Write;
#[derive(PartialEq)]
pub struct CandidateDisplay {
    candidates: Vec<String>,
}

impl CandidateDisplay {
    pub fn new() -> CandidateDisplay {
        CandidateDisplay { candidates: vec![] }
    }

    pub fn handle_input(&mut self, term: &mut Term) -> anyhow::Result<CandidateDisplayState> {
        write!(term, "> ")?;
        term.flush()?;

        let name = term.read_line()?;

        return match name.as_str() {
            "quit" => Ok(CandidateDisplayState::Done),
            _ => {
                self.candidates.push(name.to_string());
                Ok(CandidateDisplayState::Reading)
            }
        };
    }

    pub fn candidates(&self) -> &Vec<String> {
        &self.candidates
    }
}

pub enum CandidateDisplayState {
    Reading,
    Done,
}
