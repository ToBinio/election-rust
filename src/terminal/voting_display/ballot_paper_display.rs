use crate::voting::ballot::BallotPaper;

use crate::utils::elapsed_text;
use crate::voting::Voting;
use console::{style, Key, Term};
use std::io::Write;

pub struct BallotPaperDisplay {
    pub current_index: usize,
}

impl BallotPaperDisplay {
    pub fn new() -> BallotPaperDisplay {
        BallotPaperDisplay { current_index: 0 }
    }

    /// returns (how many elements above center, offset)
    pub fn get_list_offset(&self, term: &Term, voting: &Voting) -> (usize, usize) {
        let height = term.size().0 as usize;

        //todo not for 4
        let visible_papers = height / (voting.allowed_votes + 2);
        let above = visible_papers / 2;

        (
            above,
            (self.current_index as i32 - above as i32).max(0) as usize,
        )
    }

    pub fn display(
        &mut self,
        term: &mut Term,
        start_x: usize,
        width: usize,
        voting: &Voting,
    ) -> anyhow::Result<()> {
        term.move_cursor_to(start_x, 0)?;

        let (_, offset) = self.get_list_offset(term, voting);

        let height = term.size().0 as usize;

        let mut y = 0;

        for (index, paper) in voting.papers[(offset)..].iter().enumerate() {
            if y > height {
                break;
            }

            term.move_cursor_to(start_x, y)?;
            if paper.disabled {
                write!(
                    term,
                    "{}",
                    style(format!("paper {}", index + offset)).dim().white()
                )?;
            } else {
                write!(term, "{}", style(format!("paper {}", index + offset)))?;
            }
            y += 1;

            for name in &paper.voting {
                term.move_cursor_to(start_x, y)?;
                if paper.disabled {
                    write!(term, "{}", style(elapsed_text(name, width)).dim().white())?;
                } else {
                    write!(term, "{}", elapsed_text(name, width))?;
                }
                y += 1;
            }

            y += 1;
        }

        Ok(())
    }

    pub fn handle_keys(&mut self, key: &Key, voting: &mut Voting) {
        match key {
            Key::ArrowUp => {
                self.current_index += voting.papers.len();
                self.current_index -= 1;
                self.current_index %= voting.papers.len();
            }
            Key::ArrowDown => {
                self.current_index += 1;
                self.current_index %= voting.papers.len();
            }
            Key::Del => {
                voting.disable_vote(self.current_index);
            }
            _ => {}
        };
    }
}
