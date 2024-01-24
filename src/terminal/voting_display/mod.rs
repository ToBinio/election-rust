use crate::terminal::voting_display::ballot_paper_display::BallotPaperDisplay;
use crate::terminal::voting_display::candidate_selection_display::CandidateSelectionDisplay;
use crate::utils::elapsed_text;

use crate::voting::Voting;

use console::{style, Key, Term};
use std::io::Write;
use std::process::exit;

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
        let _ = ctrlc::set_handler(|| {
            let term = Term::stdout();
            term.clear_last_lines(term.size().0 as usize).unwrap();
            term.show_cursor().unwrap();

            exit(0);
        });

        VotingDisplay {
            term: Term::buffered_stdout(),
            voting,
            candidate_selection_display: CandidateSelectionDisplay::new(),
            ballot_display: BallotPaperDisplay::new(),
            mode: VotingDisplayMode::New,
        }
    }

    pub fn handle_input(&mut self) -> anyhow::Result<VotingDisplayState> {
        self.term.clear_last_lines(self.term.size().0 as usize)?;

        let width = self.term.size().1 as usize;
        let width_per = width / 3;
        let width = (width_per - 5).max(0);

        self.display_candidates(0, width)?;
        self.candidate_selection_display
            .display(&mut self.term, width_per, width, &self.voting)?;
        self.ballot_display
            .display(&mut self.term, width_per * 2, width, &self.voting)?;

        self.position_cursor(width_per)?;

        self.term.flush()?;

        self.handle_key()?;

        Ok(VotingDisplayState::Voting)
    }

    fn handle_key(&mut self) -> anyhow::Result<()> {
        let key = self.term.read_key()?;

        match (
            &self.mode,
            key,
            self.candidate_selection_display.is_on_done(&self.voting),
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

        self.voting.save();

        Ok(())
    }

    pub fn position_cursor(&mut self, width_per: usize) -> anyhow::Result<()> {
        match self.mode {
            VotingDisplayMode::New => {
                if self.candidate_selection_display.is_on_done(&self.voting) {
                    self.term
                        .move_cursor_to(width_per, 2 * self.voting.allowed_votes)?;
                } else {
                    self.term.move_cursor_to(
                        width_per
                            + self
                                .candidate_selection_display
                                .current_search_width(&self.voting),
                        self.candidate_selection_display.current_index * 2 + 1,
                    )?;
                }
            }
            VotingDisplayMode::Edit => {
                let (above, offset) = self
                    .ballot_display
                    .get_list_offset(&self.term, &self.voting);

                if offset != 0 {
                    self.term
                        .move_cursor_to(width_per * 2, (self.voting.allowed_votes + 2) * above)?;
                } else {
                    self.term.move_cursor_to(
                        width_per * 2,
                        (self.voting.allowed_votes + 2) * self.ballot_display.current_index,
                    )?;
                }
            }
        }

        Ok(())
    }

    fn display_candidates(&mut self, start_x: usize, width: usize) -> anyhow::Result<()> {
        self.term.move_cursor_to(start_x, 0)?;
        writeln!(self.term, "{}", style("Candidates").bold())?;
        writeln!(self.term, "{} Invalid", self.voting.invalid())?;

        for (index, candidate) in self.voting.candidates.iter().enumerate() {
            self.term.move_cursor_to(0, index + 2)?;

            write!(
                self.term,
                "{}|{} {}",
                style(candidate.get_votes()).red(),
                candidate.get_first_votes(),
                elapsed_text(&candidate.name, width),
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
