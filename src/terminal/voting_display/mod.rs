use crate::terminal::voting_display::ballot_paper_display::BallotPaperDisplay;
use crate::terminal::voting_display::candidate_selection_display::{
    CandidateSelection, CandidateSelectionDisplay,
};
use crate::utils::elepesed_text;

use crate::voting::Voting;

use console::{style, Key, Term};
use std::io::Write;

pub mod candidate_selection_display;

pub mod ballot_paper_display;

pub struct VotingDisplay {
    voting: Voting,

    candidate_selection_display: CandidateSelectionDisplay,
    ballot_display: BallotPaperDisplay,

    term: Term,

    mode: VotingDisplayMode,
}

impl VotingDisplay {
    pub fn new(voting: Voting) -> VotingDisplay {
        let mut candidate_selection_display = CandidateSelectionDisplay::new();

        candidate_selection_display
            .add_candidate_selection(CandidateSelection::new("First".to_string()));
        candidate_selection_display
            .add_candidate_selection(CandidateSelection::new("Second".to_string()));

        VotingDisplay {
            term: Term::buffered_stdout(),
            voting,
            candidate_selection_display,
            ballot_display: BallotPaperDisplay::new(),
            mode: VotingDisplayMode::New,
        }
    }

    pub fn handle_input(&mut self) -> anyhow::Result<VotingDisplayState> {
        self.term.clear_last_lines(self.term.size().0 as usize)?;

        self.display_candidates(0, 20)?;
        self.candidate_selection_display.display(
            &mut self.term,
            25,
            20,
            &self.voting.candidates,
        )?;
        self.ballot_display
            .display(&mut self.term, 50, 25, &self.voting.papers)?;

        self.position_cursor()?;

        self.term.flush()?;

        self.handle_key()?;

        Ok(VotingDisplayState::Voting)
    }

    fn handle_key(&mut self) -> anyhow::Result<()> {
        let key = self.term.read_key()?;

        match (
            &self.mode,
            key,
            self.candidate_selection_display.is_on_done(),
        ) {
            (VotingDisplayMode::New, Key::ArrowRight, _)
            | (VotingDisplayMode::New, Key::ArrowLeft, _) => self.mode = VotingDisplayMode::Edit,
            (VotingDisplayMode::Edit, Key::ArrowRight, _)
            | (VotingDisplayMode::Edit, Key::ArrowLeft, _) => self.mode = VotingDisplayMode::New,

            (VotingDisplayMode::Edit, key, _) => {
                self.ballot_display.handle_keys(&key, &mut self.voting)
            }

            (VotingDisplayMode::New, key, _) => self
                .candidate_selection_display
                .handle_keys(&key, &mut self.voting)?,
        }
        Ok(())
    }

    pub fn position_cursor(&mut self) -> anyhow::Result<()> {
        match self.mode {
            VotingDisplayMode::New => {
                if self.candidate_selection_display.is_on_done() {
                    self.term
                        .move_cursor_to(25, 2 * self.candidate_selection_display.len())?;
                } else {
                    self.term.move_cursor_to(
                        25 + self.candidate_selection_display.current_search_width(),
                        self.candidate_selection_display.current_index * 2 + 1,
                    )?;
                }
            }
            VotingDisplayMode::Edit => {
                let (above, offset) = self.ballot_display.get_list_offset(&self.term);

                if offset != 0 {
                    self.term.move_cursor_to(50, 4 * above)?;
                } else {
                    self.term
                        .move_cursor_to(50, 4 * self.ballot_display.current_index)?;
                }
            }
        }

        Ok(())
    }

    fn display_candidates(&mut self, start_x: usize, _width: usize) -> anyhow::Result<()> {
        self.term.move_cursor_to(start_x, 0)?;
        writeln!(self.term, "{}", style("Candidates").bold())?;
        writeln!(self.term, "{} Invalid", self.voting.invalid())?;

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
