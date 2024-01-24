use crate::voting::ballot::BallotPaper;
use crate::voting::candidate::Candidate;
use crate::voting::candidate_selection::CandidateSelection;
use anyhow::bail;
use console::style;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::string::ToString;

pub mod candidate;

pub mod ballot;

pub mod candidate_selection;

static SELECTION_HEADER: &'static [&'static str] = &["First", "Second", "Third", "Fourth"];

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
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
    ) -> anyhow::Result<Voting> {
        if allowed_votes > 4 {
            bail!("vote count has to be lower than 4");
        }

        let mut candidate_selections = vec![];

        for index in 0..allowed_votes {
            candidate_selections.push(CandidateSelection::new(SELECTION_HEADER[index].to_string()))
        }

        Ok(Voting {
            candidate_selections,
            candidates,
            papers: vec![],
            invalid_vote_count: 0,
            allowed_votes,
            save_path: save_path.as_ref().to_str().unwrap().to_string(),
        })
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

#[cfg(test)]
mod tests {
    use crate::voting::ballot::BallotPaper;
    use crate::voting::candidate::Candidate;
    use crate::voting::Voting;
    use std::env::temp_dir;
    use std::fs;

    fn get_candidates(size: usize) -> [Candidate; 3] {
        [
            Candidate::new("time test".to_string(), size),
            Candidate::new("test".to_string(), size),
            Candidate::new("ok i think".to_string(), size),
        ]
    }

    #[test]
    fn constructor() {
        let voting = Voting::new(Vec::from(get_candidates(4)), "test.txt", 4).unwrap();

        assert_eq!(voting.allowed_votes, 4);
        assert_eq!(voting.invalid_vote_count, 0);
        assert_eq!(voting.save_path, "test.txt");
        assert_eq!(voting.candidates, Vec::from(get_candidates(4)));
    }

    #[test]
    fn load() {
        let temp_dir = temp_dir();
        let save_path = temp_dir.join("save.json");

        let voting_a = Voting::new(Vec::from(get_candidates(4)), &save_path, 4).unwrap();

        voting_a.save();

        let voting_b = Voting::load(fs::read_to_string(save_path).unwrap()).unwrap();

        assert_eq!(voting_a, voting_b);
    }

    #[test]
    fn clear() {
        let mut voting = Voting::new(Vec::from(get_candidates(4)), "test.txt", 4).unwrap();

        for selection in &mut voting.candidate_selections {
            selection.search_text = "test".to_string();
        }

        for selection in &mut voting.candidate_selections {
            assert!(!selection.search_text.is_empty())
        }

        voting.clear_selections();

        for selection in &mut voting.candidate_selections {
            assert!(selection.search_text.is_empty())
        }
    }

    #[test]
    fn vote() {
        let mut voting = Voting::new(Vec::from(get_candidates(2)), "test.txt", 2).unwrap();

        voting.candidate_selections[0].search_text = "test".to_string();
        voting.candidate_selections[1].search_text = "ok".to_string();

        voting.vote();

        for selection in &mut voting.candidate_selections {
            assert!(selection.search_text.is_empty())
        }

        assert_eq!(voting.invalid(), 0);
        assert_eq!(voting.papers.len(), 1);
    }

    #[test]
    fn unvote() {
        let mut voting = Voting::new(Vec::from(get_candidates(2)), "test.txt", 2).unwrap();

        voting.candidate_selections[0].search_text = "test".to_string();
        voting.candidate_selections[1].search_text = "ok".to_string();

        voting.vote();
        voting.disable_vote(0);

        for selection in &mut voting.candidate_selections {
            assert!(selection.search_text.is_empty())
        }

        assert_eq!(voting.invalid(), 0);

        assert_eq!(voting.papers.len(), 1);
        assert_eq!(
            voting.papers.iter().filter(|paper| !paper.disabled).count(),
            0
        );
    }
}
