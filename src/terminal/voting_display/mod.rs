use crate::terminal::voting_display::candidate_selection_display::CandidateSelectionDisplay;
use anyhow::anyhow;
use console::{style, Key, Term};
use std::io::Write;

pub mod candidate_selection_display;

pub struct VotingDisplay {
    candidates: Vec<String>,
    candidate_selections: Vec<CandidateSelectionDisplay>,
    candidate_selections_index: usize,
    term: Term,
}

impl VotingDisplay {
    pub fn new(candidates: Vec<String>) -> VotingDisplay {
        VotingDisplay {
            term: Term::buffered_stdout(),
            candidates,
            candidate_selections: vec![
                CandidateSelectionDisplay::new("First".to_string()),
                CandidateSelectionDisplay::new("Second".to_string()),
            ],
            candidate_selections_index: 0,
        }
    }

    pub fn handle_input(&mut self) -> anyhow::Result<VotingDisplayState> {
        self.term.clear_last_lines(self.term.size().0 as usize)?;

        self.display_candidates(0, 20)?;

        for (index, candidate_selection_display) in self.candidate_selections.iter().enumerate() {
            candidate_selection_display.display(
                &mut self.term,
                25,
                index * 2,
                20,
                &candidate_selection_display
                    .selected_candidates(&self.candidates)
                    .unwrap_or("".to_string()),
            )?;
        }

        let selected_selection = self
            .candidate_selections
            .get_mut(self.candidate_selections_index)
            .ok_or(anyhow!("invalid selection index"))?;

        self.term.move_cursor_to(
            25 + selected_selection.search_width(),
            self.candidate_selections_index * 2 + 1,
        )?;

        self.term.flush()?;

        let key = self.term.read_key()?;

        match key {
            Key::Enter => {
                self.candidate_selections_index += 1;
                self.candidate_selections_index %= self.candidate_selections.len();
            }
            _ => {
                selected_selection.handle_keys(&key, &self.candidates);
            }
        }

        Ok(VotingDisplayState::Voting)
    }

    fn display_candidates(&mut self, start_x: usize, width: usize) -> anyhow::Result<()> {
        self.term.move_cursor_to(start_x, 0)?;
        write!(self.term, "{}", style("Candidates").bold())?;

        for (index, name) in self.candidates.iter().enumerate().map(|(index, name)| {
            if name.len() > width {
                (index, format!("{}...", &name[..17]))
            } else {
                (index, name.to_string())
            }
        }) {
            self.term.move_cursor_to(0, index + 1)?;
            write!(self.term, "{}", name)?;
        }

        Ok(())
    }
}

#[derive(PartialEq)]
pub enum VotingDisplayState {
    Voting,
    Done,
}
