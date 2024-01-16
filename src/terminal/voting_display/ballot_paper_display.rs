use clap::builder::Str;
use console::Term;
use std::io::Write;

pub struct BallotPaper {
    voting: Vec<String>,
}

impl BallotPaper {
    pub fn new(voting: Vec<String>) -> BallotPaper {
        BallotPaper { voting }
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

    pub fn display(&mut self, term: &mut Term, start_x: usize, width: usize) -> anyhow::Result<()> {
        term.move_cursor_to(start_x, 0)?;

        let mut y = 0;

        for (usize, paper) in self.papers.iter().enumerate() {
            term.move_cursor_to(start_x, y)?;
            write!(term, "paper {}", usize)?;
            y += 1;

            for name in &paper.voting {
                term.move_cursor_to(start_x, y)?;
                write!(term, "{}", name)?;
                y += 1;
            }

            y += 1;
        }

        Ok(())
    }
}
