use crate::voting::candidate::Candidate;
use crate::voting::candidate_selection::CandidateSelection;

pub mod candidate;
pub mod candidate_selection;

pub mod ballot;

pub struct Voting {
    pub candidates: Vec<Candidate>,
    pub candidate_selections: Vec<CandidateSelection>,
    invalid_count: usize,
}

impl Voting {
    pub fn new(candidates: Vec<Candidate>) -> Voting {
        Voting {
            candidates,
            candidate_selections: vec![],
            invalid_count: 0,
        }
    }

    pub fn invalid(&self) -> usize {
        self.invalid_count
    }

    pub fn vote_invalid(&mut self) {
        self.invalid_count += 1
    }

    pub fn unvote_invalid(&mut self) {
        self.invalid_count -= 1
    }
}
