use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Candidate {
    pub name: String,
    votes: Vec<usize>,
}

impl Candidate {
    pub fn new(name: String, size: usize) -> Candidate {
        Candidate {
            name,
            votes: vec![0; size],
        }
    }

    pub fn vote(&mut self, level: usize) {
        self.votes[level] += 1;
    }
    pub fn unvote(&mut self, level: usize) {
        self.votes[level] -= 1;
    }

    pub fn get_first_votes(&self) -> usize {
        self.votes[0]
    }

    pub fn get_votes(&self) -> usize {
        let size = self.votes.len();

        self.votes
            .iter()
            .enumerate()
            .map(|(index, count)| (size - index) * count)
            .sum()
    }
}
