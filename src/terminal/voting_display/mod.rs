use crate::terminal::voting_display::ballot_paper_display::BallotPaperDisplay;
use crate::terminal::voting_display::candidate_selection_display::CandidateSelectionDisplay;
use crate::utils::{elepesed_text, get_fitting_names};
use crate::voting::ballot::BallotPaper;
use crate::voting::candidate::Candidate;
use crate::voting::Voting;
use anyhow::anyhow;
use console::{style, Key, Style, Term};
use std::io::Write;
use std::ops::Rem;

pub mod candidate_selection_display;

pub mod ballot_paper_display;

pub struct VotingDisplay {
    voting: Voting,
    candidate_selections: Vec<CandidateSelectionDisplay>,
    candidate_selections_index: usize,

    ballot_display: BallotPaperDisplay,
    ballot_selection_index: usize,

    term: Term,

    mode: VotingDisplayMode,
}

impl VotingDisplay {
    pub fn new(voting: Voting) -> VotingDisplay {
        VotingDisplay {
            term: Term::buffered_stdout(),
            voting,
            candidate_selections: vec![
                CandidateSelectionDisplay::new("First".to_string()),
                CandidateSelectionDisplay::new("Second".to_string()),
            ],
            candidate_selections_index: 0,
            ballot_display: BallotPaperDisplay::new(),
            mode: VotingDisplayMode::New,
            ballot_selection_index: 0,
        }
    }

    pub fn handle_input(&mut self) -> anyhow::Result<VotingDisplayState> {
        self.term.clear_last_lines(self.term.size().0 as usize)?;

        self.display_candidates(0, 20)?;

        for (index, candidate_selection_display) in self.candidate_selections.iter().enumerate() {
            let is_valid = candidate_selection_display.is_valid(
                &self.candidate_selections,
                &self.voting.candidates,
                index,
            );

            candidate_selection_display.display(
                &mut self.term,
                25,
                index * 2,
                20,
                &candidate_selection_display
                    .selected_candidate(&self.voting.candidates)
                    .unwrap_or("".to_string()),
                is_valid,
            )?;
        }

        self.ballot_display
            .display(&mut self.term, 50, 25, self.ballot_selection_index)?;

        self.position_cursor()?;

        self.term.flush()?;

        self.handle_key()?;

        Ok(VotingDisplayState::Voting)
    }

    fn handle_key(&mut self) -> anyhow::Result<()> {
        let key = self.term.read_key()?;

        match (&self.mode, key, self.is_on_done()) {
            (VotingDisplayMode::New, Key::ArrowRight, _)
            | (VotingDisplayMode::New, Key::ArrowLeft, _) => self.mode = VotingDisplayMode::Edit,
            (VotingDisplayMode::Edit, Key::ArrowRight, _)
            | (VotingDisplayMode::Edit, Key::ArrowLeft, _) => self.mode = VotingDisplayMode::New,
            (VotingDisplayMode::New, Key::Enter, _) => {
                self.candidate_selections_index += 1;
                self.candidate_selections_index %= self.candidate_selections.len() + 1;
            }
            (VotingDisplayMode::Edit, Key::ArrowUp, _) => {
                self.ballot_selection_index += self.ballot_display.len();
                self.ballot_selection_index -= 1;
                self.ballot_selection_index %= self.ballot_display.len();
            }
            (VotingDisplayMode::Edit, Key::ArrowDown, _) => {
                self.ballot_selection_index += 1;
                self.ballot_selection_index %= self.ballot_display.len();
            }
            (VotingDisplayMode::Edit, Key::Del, _) => {
                self.ballot_display.disable(self.ballot_selection_index);

                let paper = self.ballot_display.get_paper(self.ballot_selection_index);

                if paper.invalid {
                    self.voting.unvote_invalid();
                } else {
                    for (index, vote) in paper.voting.iter().enumerate() {
                        if let Some(candidate) = self
                            .voting
                            .candidates
                            .iter_mut()
                            .find(|candidate| &candidate.name == vote)
                        {
                            candidate.unvote(index)
                        }
                    }
                }
            }
            (VotingDisplayMode::New, key, false) => {
                let selected_selection = self
                    .candidate_selections
                    .get_mut(self.candidate_selections_index)
                    .ok_or(anyhow!("invalid selection index"))?;

                selected_selection.handle_keys(&key, &self.voting.candidates);
            }
            (VotingDisplayMode::New, Key::Char(' '), true) => {
                let in_valid = self
                    .candidate_selections
                    .iter()
                    .enumerate()
                    .find(|(index, selection)| {
                        !selection.is_valid(
                            &self.candidate_selections,
                            &self.voting.candidates,
                            *index,
                        )
                    })
                    .is_some();

                if in_valid {
                    self.voting.vote_invalid();
                } else {
                    for (index, display) in self.candidate_selections.iter_mut().enumerate() {
                        if let Some(name) = display.selected_candidate(&self.voting.candidates) {
                            if let Some(candidate) = self
                                .voting
                                .candidates
                                .iter_mut()
                                .find(|candidate| &candidate.name == &name)
                            {
                                candidate.vote(index)
                            }
                        }
                    }
                }

                self.ballot_display.add_paper(BallotPaper::new(
                    self.candidate_selections
                        .iter()
                        .map(|candidate| {
                            candidate
                                .selected_candidate(&self.voting.candidates)
                                .unwrap_or("".to_string())
                        })
                        .collect(),
                    in_valid,
                ));

                for display in &mut self.candidate_selections {
                    display.clear();
                }
            }
            (_, _, _) => {}
        }
        Ok(())
    }

    pub fn is_on_done(&self) -> bool {
        self.candidate_selections_index == self.candidate_selections.len()
    }

    pub fn position_cursor(&mut self) -> anyhow::Result<()> {
        match self.mode {
            VotingDisplayMode::New => {
                if self.is_on_done() {
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
            }
            VotingDisplayMode::Edit => {
                let (above, offset) =
                    BallotPaperDisplay::get_list_offset(&self.term, self.ballot_selection_index);

                if offset != 0 {
                    self.term.move_cursor_to(50, 4 * above)?;
                } else {
                    self.term
                        .move_cursor_to(50, 4 * self.ballot_selection_index)?;
                }
            }
        }

        Ok(())
    }

    fn display_candidates(&mut self, start_x: usize, width: usize) -> anyhow::Result<()> {
        self.term.move_cursor_to(start_x, 0)?;
        writeln!(self.term, "{}", style("Candidates").bold())?;
        writeln!(self.term, "{} {}", self.voting.invalid(), "Invalid")?;

        for (index, candidate) in self.voting.candidates.iter().enumerate() {
            self.term.move_cursor_to(0, index + 2)?;

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

#[derive(PartialEq)]
pub enum VotingDisplayMode {
    New,
    Edit,
}
