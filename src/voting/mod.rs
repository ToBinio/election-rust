use crate::voting::ballot::BallotPaper;
use crate::voting::candidate::Candidate;

pub mod candidate;

pub mod ballot;

pub struct Voting {
    pub candidates: Vec<Candidate>,
    pub papers: Vec<BallotPaper>,
    invalid_count: usize,
}

impl Voting {
    pub fn new(candidates: Vec<Candidate>) -> Voting {
        Voting {
            candidates,
            papers: vec![],
            invalid_count: 0,
        }
    }

    pub fn vote(&mut self, votes: Option<Vec<String>>) {
        match votes {
            None => {
                self.papers.push(BallotPaper::new(
                    vec!["invalid".to_string(), "invalid".to_string()],
                    true,
                ));
                self.invalid_count += 1;
            }
            Some(votes) => {
                'outer: for (index, vote) in votes.iter().enumerate() {
                    for candidate in &mut self.candidates {
                        if candidate.name == *vote {
                            candidate.vote(index);
                            continue 'outer;
                        }
                    }
                }

                self.papers.push(BallotPaper::new(votes, false));
            }
        }
    }

    pub fn disable_vote(&mut self, index: usize) {
        self.papers[index].disabled = true;

        let paper = &self.papers[index];

        if paper.invalid {
            self.invalid_count -= 1;
        } else {
            for (index, vote) in paper.voting.iter().enumerate() {
                if let Some(candidate) = self
                    .candidates
                    .iter_mut()
                    .find(|candidate| &candidate.name == vote)
                {
                    candidate.unvote(index)
                }
            }
        }
    }

    pub fn invalid(&self) -> usize {
        self.invalid_count
    }
}
