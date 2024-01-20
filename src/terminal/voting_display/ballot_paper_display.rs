use crate::voting::ballot::BallotPaper;
use console::{style, Term};
use std::io::Write;

pub struct BallotPaperDisplay {
    papers: Vec<BallotPaper>,
}

impl BallotPaperDisplay {
    pub fn new() -> BallotPaperDisplay {
        BallotPaperDisplay { papers: vec![] }
    }

    pub fn add_paper(&mut self, paper: BallotPaper) {
        self.papers.push(paper);
    }

    pub fn disable(&mut self, index: usize) {
        self.papers[index].disabled = true;
    }

    pub fn get_paper(&self, index: usize) -> &BallotPaper {
        &self.papers[index]
    }

    pub fn len(&self) -> usize {
        self.papers.len()
    }

    /// returns (how many elements above center, offset)
    pub fn get_list_offset(term: &Term, current_index: usize) -> (usize, usize) {
        let height = term.size().0 as usize;

        //todo not for 4
        let visible_papers = height / 4;
        let above = visible_papers / 2;

        (above, (current_index as i32 - above as i32).max(0) as usize)
    }

    pub fn display(
        &mut self,
        term: &mut Term,
        start_x: usize,
        _width: usize,
        current_index: usize,
    ) -> anyhow::Result<()> {
        term.move_cursor_to(start_x, 0)?;

        let (_, offset) = BallotPaperDisplay::get_list_offset(term, current_index);

        let height = term.size().0 as usize;

        let mut y = 0;

        for (index, paper) in self.papers[(offset)..].iter().enumerate() {
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
                    write!(term, "{}", style(name).dim().white())?;
                } else {
                    write!(term, "{}", style(name))?;
                }
                y += 1;
            }

            y += 1;
        }

        Ok(())
    }
}
