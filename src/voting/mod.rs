use crate::voting::ballot::BallotPaper;
use crate::voting::candidate::Candidate;
use crate::voting::candidate_selection::CandidateSelection;
use console::style;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::string::ToString;

pub mod candidate;

pub mod ballot;

pub mod candidate_selection;

static SELECTION_HEADER: &'static [&'static str] = &["First", "Second", "Third", "Fourth"];

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
    pub fn new<P: AsRef<Path>>(
        candidates: Vec<Candidate>,
        save_path: P,
        allowed_votes: usize,
    ) -> Voting {
        let mut candidate_selections = vec![];

        for index in 0..allowed_votes {
            candidate_selections.push(CandidateSelection::new(SELECTION_HEADER[index].to_string()))
        }

        Voting {
            candidate_selections,
            candidates,
            papers: vec![],
            invalid_vote_count: 0,
            allowed_votes,
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

    pub fn vote(&mut self) {
        let votes: Vec<(bool, String)> = self
            .candidate_selections
            .iter()
            .enumerate()
            .map(|(index, selection)| {
                (
                    selection.is_valid(&self.candidate_selections, &self.candidates, index),
                    selection,
                )
            })
            .map(|(valid, selection)| {
                if !valid {
                    return (valid, style("invalid").white().dim().to_string());
                }

                (
                    valid,
                    selection
                        .selected_candidate(&self.candidates)
                        .unwrap_or(style("invalid").white().dim().to_string()),
                )
            })
            .collect();

        let is_valid = votes.iter().any(|(valid, _)| *valid);

        self.papers.push(BallotPaper::new(
            votes.iter().map(|(_, text)| text.to_string()).collect(),
            !is_valid,
        ));

        if is_valid {
            'outer: for (index, (_, vote)) in votes.iter().enumerate() {
                for candidate in &mut self.candidates {
                    if candidate.name == *vote {
                        candidate.vote(index);
                        continue 'outer;
                    }
                }
            }
        } else {
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
