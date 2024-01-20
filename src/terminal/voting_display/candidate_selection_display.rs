use crate::terminal::voting_display::VotingDisplayMode;
use crate::utils::get_fitting_names;
use crate::voting::candidate::Candidate;
use crate::voting::Voting;
use anyhow::anyhow;
use console::{style, Key, Term};
use std::io::Write;

pub struct CandidateSelectionDisplay {
    pub candidate_selections: Vec<CandidateSelection>,
    pub current_index: usize,
}

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

    fn possible_candidates_names(&self, candidates: &[Candidate]) -> Vec<String> {
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

    pub fn search_width(&self) -> usize {
        self.search_text.len()
    }

    pub fn handle_keys(&mut self, key: &Key, candidates: &[Candidate]) {
        let previews = self.possible_candidates_names(candidates);

        match key {
            Key::Tab | Key::Char('\t') => {
                self.selected_preview += 1;
                self.selected_preview %= previews.len();
            }
            Key::Backspace => {
                self.search_text.pop();
            }
            Key::Char(char) => {
                self.search_text += &char.to_string();

                if previews.len() == 0 {
                    self.selected_preview = 0;
                } else {
                    self.selected_preview %= previews.len();
                }
            }
            _ => {}
        }
    }
}

impl CandidateSelectionDisplay {
    pub fn new() -> CandidateSelectionDisplay {
        CandidateSelectionDisplay {
            candidate_selections: vec![],
            current_index: 0,
        }
    }

    pub fn add_candidate_selection(&mut self, candidate_selection: CandidateSelection) {
        self.candidate_selections.push(candidate_selection);
    }

    fn is_valid(&self, candidates: &Vec<Candidate>) -> bool {
        let is_invalid = self
            .candidate_selections
            .iter()
            .enumerate()
            .find(|(index, selection)| {
                !selection.is_valid(&self.candidate_selections, candidates, *index)
            })
            .is_some();

        !is_invalid
    }

    pub fn get_votes(&self, candidates: &Vec<Candidate>) -> Option<Vec<String>> {
        if !self.is_valid(candidates) {
            return None;
        }

        Some(
            self.candidate_selections
                .iter()
                .filter_map(|selection| selection.selected_candidate(candidates))
                .collect(),
        )
    }

    pub fn current_search_width(&self) -> usize {
        self.candidate_selections[self.current_index].search_width()
    }

    pub fn len(&self) -> usize {
        self.candidate_selections.len()
    }

    pub fn clear(&mut self) {
        for selection in &mut self.candidate_selections {
            selection.clear();
        }
    }

    pub fn display(
        &self,
        term: &mut Term,
        start_x: usize,
        _width: usize,
        candidates: &Vec<Candidate>,
    ) -> anyhow::Result<()> {
        for (index, candidate_selection) in self.candidate_selections.iter().enumerate() {
            let is_valid =
                candidate_selection.is_valid(&self.candidate_selections, &candidates, index);

            let y = index * 2;
            let preview = candidate_selection
                .selected_candidate(candidates)
                .unwrap_or("".to_string());

            term.move_cursor_to(start_x, y)?;
            write!(term, "{}", style(&candidate_selection.header).bold())?;

            if !is_valid {
                write!(term, " ❌")?;
            }

            term.move_cursor_to(start_x, y + 1)?;
            write!(term, "{}", style(preview).green())?;

            term.move_cursor_to(start_x, y + 1)?;
            write!(term, "{}", candidate_selection.search_text)?;
        }

        //render done-button
        if self.is_on_done() {
            term.move_cursor_to(25, 2 * self.candidate_selections.len())?;

            write!(term, "{}", style("Done").on_yellow().bold())?;
        } else {
            term.move_cursor_to(25, 2 * self.candidate_selections.len())?;
            write!(term, "{}", style("Done").yellow().bold())?;

            let selected_selection = self
                .candidate_selections
                .get(self.current_index)
                .ok_or(anyhow!("invalid selection index"))?;

            term.move_cursor_to(
                25 + selected_selection.search_width(),
                self.current_index * 2 + 1,
            )?;
        }

        Ok(())
    }

    pub fn is_on_done(&self) -> bool {
        self.current_index == self.candidate_selections.len()
    }

    pub fn handle_keys(&mut self, key: &Key, voting: &mut Voting) -> anyhow::Result<()> {
        match (key, self.is_on_done()) {
            (Key::Enter, _) => {
                self.current_index += 1;
                self.current_index %= self.candidate_selections.len() + 1;
            }
            (Key::Char(' '), true) => {
                voting.vote(self.get_votes(&voting.candidates));

                self.clear();
            }
            (key, false) => {
                self.candidate_selections
                    .get_mut(self.current_index)
                    .unwrap()
                    .handle_keys(key, &voting.candidates);
            }
            _ => {}
        }

        Ok(())
    }
}
