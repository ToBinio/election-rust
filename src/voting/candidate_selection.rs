use crate::utils::get_fitting_names;
use crate::voting::candidate::Candidate;

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
