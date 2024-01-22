use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Candidate {
    pub name: String,
    pub votes: Vec<usize>,
}

impl Candidate {
    pub fn  new(name: String, size: usize) -> Candidate {
        Candidate {
            name,
            votes: vec![0; size],
        }
    }

    pub fn vote(&mut self, level: usize) {
        *self.votes.get_mut(level).unwrap() += 1;
    }
    pub fn unvote(&mut self, level: usize) {
        *self.votes.get_mut(level).unwrap() -= 1;
    }

    pub fn get_first_votes(&self) -> usize {
        *self.votes.get(0).unwrap()
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

#[cfg(test)]
mod tests {
    use crate::voting::candidate::Candidate;

    #[test]
    fn voting() {
        let mut candidate = Candidate::new("test".to_string(), 4);

        candidate.vote(1);

        candidate.vote(3);
        candidate.vote(3);

        candidate.vote(0);

        assert_eq!(candidate.get_first_votes(), 1);
        assert_eq!(candidate.get_votes(), 1 + 1 + 3 + 4);
    }

    #[test]
    fn unvote() {
        let mut candidate = Candidate::new("test".to_string(), 4);

        candidate.vote(1);

        candidate.vote(0);
        candidate.unvote(0);

        candidate.vote(3);

        assert_eq!(candidate.get_first_votes(), 0);
        assert_eq!(candidate.get_votes(), 3 + 1);
    }
}
