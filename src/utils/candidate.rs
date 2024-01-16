use std::fs;
use std::path::Path;

pub struct Candidate {
    pub name: String,
    votes: Vec<usize>,
}

impl Candidate {
    pub fn new(name: String, size: usize) -> Candidate {
        Candidate {
            name,
            votes: Vec::with_capacity(size),
        }
    }

    pub fn vote(&mut self, level: usize) {
        self.votes[level] += 1;
    }

    pub fn get_sum(&self) -> usize {
        let size = self.votes.len();

        self.votes
            .iter()
            .enumerate()
            .map(|(index, count)| (size - index) * count)
            .sum()
    }
}

pub const FILE_PATH: &str = "candidates.txt";

pub fn load_candidates<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Candidate>> {
    let content = fs::read_to_string(path)?;

    Ok(content
        .lines()
        .map(|name| Candidate::new(name.to_string(), 2))
        .collect())
}

pub fn save_candidates<P: AsRef<Path>>(path: P, candidates: &Vec<String>) -> anyhow::Result<()> {
    let content = candidates.join("\n").to_string();

    fs::write(path, content)?;

    Ok(())
}