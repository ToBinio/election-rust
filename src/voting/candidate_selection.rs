use crate::utils::get_fitting_names;
use crate::voting::candidate::Candidate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CandidateSelection {
    pub search_text: String,
    pub selected_preview: usize,
    pub header: String,
}

impl CandidateSelection {
    pub fn new(header: String) -> CandidateSelection {
        CandidateSelection {
            search_text: "".to_string(),
            selected_preview: 0,
            header,
        }
    }

    pub fn clear(&mut self) {
        self.search_text = "".to_string();
        self.selected_preview = 0;
    }

    pub fn is_valid(
        &self,
        others: &[CandidateSelection],
        candidates: &[Candidate],
        own_index: usize,
    ) -> bool {
        let candidate = self.selected_candidate(candidates);

        if others
            .iter()
            .enumerate()
            .filter(|(other_index, _)| own_index != *other_index)
            .any(|(_, other)| other.selected_candidate(candidates) == candidate)
        {
            return false;
        }

        self.search_text.is_empty() || !self.possible_candidates_names(candidates).is_empty()
    }

    pub fn possible_candidates_names(&self, candidates: &[Candidate]) -> Vec<String> {
        get_fitting_names(
            candidates
                .iter()
                .map(|candidate| candidate.name.to_string())
                .collect(),
            &self.search_text,
        )
    }

    pub fn selected_candidate(&self, candidates: &[Candidate]) -> Option<String> {
        if self.search_text.is_empty() {
            return None;
        }

        self.possible_candidates_names(candidates)
            .get(self.selected_preview)
            .map(|name| name.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::voting::ballot::BallotPaper;
    use crate::voting::candidate::Candidate;
    use crate::voting::candidate_selection::CandidateSelection;

    static candidates: &'static [Candidate; 3] = &[
        Candidate {
            name: "some test".to_string(),
            votes: vec![],
        },
        Candidate {
            name: "test".to_string(),
            votes: vec![],
        },
        Candidate {
            name: "ok i think".to_string(),
            votes: vec![],
        },
    ];

    #[test]
    fn constructor() {
        let selection = CandidateSelection::new("header".to_string());

        assert_eq!(selection.selected_preview, 0);
        assert_eq!(selection.header, "header".to_string());
        assert_eq!(selection.search_text, "".to_string());
    }

    #[test]
    fn clear() {
        let mut selection = CandidateSelection::new("header".to_string());

        selection.search_text = "test".to_string();
        selection.selected_preview = 5;

        selection.clear();

        assert_eq!(selection.selected_preview, 0);
        assert_eq!(selection.header, "header".to_string());
        assert_eq!(selection.search_text, "".to_string());
    }

    #[test]
    fn is_valid() {
        let mut selection_a = CandidateSelection::new("header".to_string());
        selection_a.search_text = "test".to_string();

        let mut selection_b = CandidateSelection::new("header2".to_string());
        selection_b.search_text = "ok".to_string();

        let selections = [selection_a, selection_b];

        assert!(selections[0].is_valid(&selections, candidates, 0,));
    }

    fn is_in_valid() {
        let mut selection_a = CandidateSelection::new("header".to_string());
        selection_a.search_text = "test".to_string();

        let mut selection_b = CandidateSelection::new("header2".to_string());
        selection_b.search_text = "test".to_string();

        let selections = [selection_a, selection_b];

        assert!(!selections[0].is_valid(&selections, candidates, 0,));
    }
}
