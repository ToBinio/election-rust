use crate::utils::candidate::Candidate;
use crate::utils::get_fitting_names;
use console::{style, Key, Term};
use std::io::Write;

pub struct CandidateSelectionDisplay {
    search_text: String,
    selected_preview: usize,
    header: String,
}

impl CandidateSelectionDisplay {
    pub fn new(header: String) -> CandidateSelectionDisplay {
        CandidateSelectionDisplay {
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
        others: &Vec<CandidateSelectionDisplay>,
        candidates: &Vec<Candidate>,
        own_index: usize,
    ) -> bool {
        let candidate = self.selected_candidate(candidates);

        if others
            .iter()
            .enumerate()
            .filter(|(other_index, _)| own_index != *other_index)
            .find(|(_, other)| other.selected_candidate(candidates) == candidate)
            .is_some()
        {
            return false;
        }

        self.search_text.is_empty() || self.possible_candidates_names(candidates).len() > 0
    }

    fn possible_candidates_names(&self, candidates: &Vec<Candidate>) -> Vec<String> {
        get_fitting_names(
            candidates
                .iter()
                .map(|candidate| candidate.name.to_string())
                .collect(),
            &self.search_text,
        )
    }

    pub fn selected_candidate(&self, candidates: &Vec<Candidate>) -> Option<String> {
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

    pub fn display(
        &self,
        term: &mut Term,
        start_x: usize,
        start_y: usize,
        _width: usize,
        preview: &str,
        is_valid: bool,
    ) -> anyhow::Result<()> {
        term.move_cursor_to(start_x, start_y)?;
        write!(term, "{}", style(&self.header).bold())?;

        if !is_valid {
            write!(term, " {}", "❌")?;
        }

        term.move_cursor_to(start_x, start_y + 1)?;
        write!(term, "{}", style(preview).green())?;

        term.move_cursor_to(start_x, start_y + 1)?;
        write!(term, "{}", self.search_text)?;

        Ok(())
    }

    pub fn handle_keys(&mut self, key: &Key, candidates: &Vec<Candidate>) {
        let previews = self.possible_candidates_names(candidates);

        match key {
            Key::ArrowRight => {
                if let Some(preview_text) = self.selected_candidate(candidates) {
                    self.search_text = preview_text.to_string()
                }
            }
            Key::ArrowUp => {
                self.selected_preview += 1;
                self.selected_preview %= previews.len();
            }
            Key::ArrowDown => {
                self.selected_preview += previews.len();
                self.selected_preview -= 1;
                self.selected_preview %= previews.len();
            }
            Key::Backspace => {
                self.search_text.pop();
            }
            Key::Char(char) => {
                self.search_text += &char.to_string();
            }
            _ => {}
        }
    }
}
