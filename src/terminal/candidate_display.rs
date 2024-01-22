use crate::utils::candidate::save_candidates;
use console::Term;
use std::io::Write;
use std::path::Path;

pub struct CandidateDisplay {
    term: Term,
    candidates: Vec<String>,
    path: String,
}

impl CandidateDisplay {
    pub fn new<P: AsRef<Path>>(path: P) -> CandidateDisplay {
        CandidateDisplay {
            term: Term::buffered_stdout(),
            candidates: vec![],
            path: path.as_ref().to_str().unwrap().to_string(),
        }
    }

    pub fn handle_input(&mut self) -> anyhow::Result<CandidateDisplayState> {
        write!(self.term, "> ")?;
        self.term.flush()?;

        let name = self.term.read_line()?;

        return match name.as_str() {
            "save" => {
                save_candidates(&self.path, &self.candidates)?;
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
