use crate::utils::candidate::save_candidates;
use console::{style, Term};
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub struct CandidateDisplay {
    term: Term,
    candidates: Vec<String>,
    path: String,
}

impl CandidateDisplay {
    pub fn new<P: AsRef<Path>>(path: P) -> CandidateDisplay {
        let _ = ctrlc::set_handler(|| {
            exit(0);
        });

        CandidateDisplay {
            term: Term::stdout(),
            candidates: vec![],
            path: path.as_ref().to_str().unwrap().to_string(),
        }
    }

    pub fn handle_input(&mut self) -> anyhow::Result<CandidateDisplayState> {
        write!(self.term, "> ")?;

        let name = self.term.read_line()?;

        return match name.as_str() {
            "save" => {
                save_candidates(&self.path, &self.candidates)?;
                Ok(CandidateDisplayState::Done)
            }
            _ => {
                let name = name.trim();

                if name.is_empty() {
                    return Ok(CandidateDisplayState::Reading);
                }

                if self.candidates.contains(&name.to_string()) {
                    writeln!(self.term, "{}", style("already defined").red())?;

                    return Ok(CandidateDisplayState::Reading);
                }

                self.candidates.push(name.to_string());
                save_candidates(&self.path, &self.candidates)?;
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
