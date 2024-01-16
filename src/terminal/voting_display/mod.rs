use crate::terminal::voting_display::ballot_paper_display::{BallotPaper, BallotPaperDisplay};
use crate::terminal::voting_display::candidate_selection_display::CandidateSelectionDisplay;
use crate::utils::candidate::Candidate;
use crate::utils::{elepesed_text, get_fitting_names};
use anyhow::anyhow;
use console::{style, Key, Style, Term};
use std::io::Write;

pub mod candidate_selection_display;

pub mod ballot_paper_display;

pub struct VotingDisplay {
    candidates: Vec<Candidate>,
    candidate_selections: Vec<CandidateSelectionDisplay>,
    candidate_selections_index: usize,

    ballot_display: BallotPaperDisplay,

    term: Term,
}

impl VotingDisplay {
    pub fn new(candidates: Vec<Candidate>) -> VotingDisplay {
        VotingDisplay {
            term: Term::buffered_stdout(),
            candidates,
            candidate_selections: vec![
                CandidateSelectionDisplay::new("First".to_string()),
                CandidateSelectionDisplay::new("Second".to_string()),
            ],
            candidate_selections_index: 0,
            ballot_display: BallotPaperDisplay::new(),
        }
    }

    pub fn handle_input(&mut self) -> anyhow::Result<VotingDisplayState> {
        self.term.clear_last_lines(self.term.size().0 as usize)?;

        self.display_candidates(0, 20)?;

        for (index, candidate_selection_display) in self.candidate_selections.iter().enumerate() {
            let is_valid = candidate_selection_display.is_valid(
                &self.candidate_selections,
                &self.candidates,
                index,
            );

            candidate_selection_display.display(
                &mut self.term,
                25,
                index * 2,
                20,
                &candidate_selection_display
                    .selected_candidate(&self.candidates)
                    .unwrap_or("".to_string()),
                is_valid,
            )?;
        }

        self.ballot_display.display(&mut self.term, 50, 25)?;

        let is_on_done_button = self.candidate_selections_index == self.candidate_selections.len();

        if is_on_done_button {
            self.term
                .move_cursor_to(25, 2 * self.candidate_selections.len())?;

            write!(self.term, "{}", style("Done").on_yellow().bold())?;
        } else {
            self.term
                .move_cursor_to(25, 2 * self.candidate_selections.len())?;
            write!(self.term, "{}", style("Done").yellow().bold())?;

            let selected_selection = self
                .candidate_selections
                .get_mut(self.candidate_selections_index)
                .ok_or(anyhow!("invalid selection index"))?;

            self.term.move_cursor_to(
                25 + selected_selection.search_width(),
                self.candidate_selections_index * 2 + 1,
            )?;
        }

        self.term.flush()?;

        let key = self.term.read_key()?;

        match (key, is_on_done_button) {
            (Key::Enter, _) => {
                self.candidate_selections_index += 1;
                self.candidate_selections_index %= self.candidate_selections.len() + 1;
            }
            (key, false) => {
                let selected_selection = self
                    .candidate_selections
                    .get_mut(self.candidate_selections_index)
                    .ok_or(anyhow!("invalid selection index"))?;

                selected_selection.handle_keys(&key, &self.candidates);
            }
            (Key::Char(' '), true) => {
                self.ballot_display.add_paper(BallotPaper::new(
                    self.candidate_selections
                        .iter()
                        .map(|candidate| {
                            candidate
                                .selected_candidate(&self.candidates)
                                .unwrap_or("".to_string())
                        })
                        .collect(),
                ));

                for display in &mut self.candidate_selections {
                    display.clear();
                }
            }
            (_, _) => {}
        }

        Ok(VotingDisplayState::Voting)
    }

    fn display_candidates(&mut self, start_x: usize, width: usize) -> anyhow::Result<()> {
        self.term.move_cursor_to(start_x, 0)?;
        write!(self.term, "{}", style("Candidates").bold())?;

        for (index, candidate) in self.candidates.iter().enumerate() {
            self.term.move_cursor_to(0, index + 1)?;

            write!(
                self.term,
                "{}|{} {}",
                style(candidate.get_sum()).red(),
                candidate.get_first_votes(),
                elepesed_text(&candidate.name, 20)
            )?;
        }

        Ok(())
    }
}

#[derive(PartialEq)]
pub enum VotingDisplayState {
    Voting,
    Done,
}
