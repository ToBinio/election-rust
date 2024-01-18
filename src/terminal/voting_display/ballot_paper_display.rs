use console::{style, Term};
use std::io::Write;

pub struct BallotPaper {
    pub voting: Vec<String>,
    disabled: bool,
    pub invalid: bool,
}

impl BallotPaper {
    pub fn new(voting: Vec<String>, invalid: bool) -> BallotPaper {
        BallotPaper {
            voting,
            disabled: false,
            invalid,
        }
    }
}

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

    pub fn display(
        &mut self,
        term: &mut Term,
        start_x: usize,
        _width: usize,
    ) -> anyhow::Result<()> {
        term.move_cursor_to(start_x, 0)?;

        let mut y = 0;

        for (index, paper) in self.papers.iter().enumerate() {
            term.move_cursor_to(start_x, y)?;
            if paper.disabled {
                write!(term, "{}", style(format!("paper {}", index)).dim().white())?;
            } else {
                write!(term, "{}", style(format!("paper {}", index)))?;
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
