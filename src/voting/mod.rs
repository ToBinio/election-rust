use crate::voting::ballot::BallotPaper;
use crate::voting::candidate::Candidate;
use crate::voting::candidate_selection::CandidateSelection;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub mod candidate;

pub mod ballot;

pub mod candidate_selection;

#[derive(Deserialize, Serialize)]
pub struct Voting {
    pub candidate_selections: Vec<CandidateSelection>,

    pub candidates: Vec<Candidate>,
    pub papers: Vec<BallotPaper>,
    invalid_vote_count: usize,

    pub allowed_votes: usize,

    save_path: String,
}

impl Voting {
    pub fn new<P: AsRef<Path>>(candidates: Vec<Candidate>, save_path: P) -> Voting {
        let candidate_selections = vec![
            CandidateSelection::new("First".to_string()),
            CandidateSelection::new("Second".to_string()),
        ];

        Voting {
            candidate_selections,
            candidates,
            papers: vec![],
            invalid_vote_count: 0,
            allowed_votes: 2,
            save_path: save_path.as_ref().to_str().unwrap().to_string(),
        }
    }

    pub fn save(&self) {
        let content = serde_json::to_string(&self).unwrap();
        fs::write(&self.save_path, content).unwrap();
    }

    pub fn load(content: String) -> serde_json::Result<Voting> {
        serde_json::from_str(&content)
    }

    pub fn clear_selections(&mut self) {
        for selection in &mut self.candidate_selections {
            selection.clear()
        }
    }

    fn is_valid_selection(&self) -> bool {
        let is_invalid = self
            .candidate_selections
            .iter()
            .enumerate()
            .any(|(index, selection)| {
                !selection.is_valid(&self.candidate_selections, &self.candidates, index)
            });

        !is_invalid
    }

    pub fn vote(&mut self) {
        if self.is_valid_selection() {
            let votes: Vec<String> = self
                .candidate_selections
                .iter()
                .filter_map(|selection| selection.selected_candidate(&self.candidates))
                .collect();

            'outer: for (index, vote) in votes.iter().enumerate() {
                for candidate in &mut self.candidates {
                    if candidate.name == *vote {
                        candidate.vote(index);
                        continue 'outer;
                    }
                }
            }

            self.papers.push(BallotPaper::new(votes, false));
        } else {
            self.papers.push(BallotPaper::new(
                vec!["invalid".to_string(), "invalid".to_string()],
                true,
            ));
            self.invalid_vote_count += 1;
        }

        self.clear_selections();
    }

    pub fn disable_vote(&mut self, index: usize) {
        self.papers[index].disabled = true;

        let paper = &self.papers[index];

        if paper.invalid {
            self.invalid_vote_count -= 1;
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
        self.invalid_vote_count
    }
}
