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
    fn possible_candidates(&self, candidates: &Vec<String>) -> Vec<String> {
        get_fitting_names(candidates, &self.search_text)
    }

    pub fn selected_candidates(&self, candidates: &Vec<String>) -> Option<String> {
        self.possible_candidates(candidates)
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
        width: usize,
        preview: &str,
    ) -> anyhow::Result<()> {
        term.move_cursor_to(start_x, start_y)?;
        write!(term, "{}", style(&self.header).bold())?;

        if self.search_text.len() > 0 {
            term.move_cursor_to(start_x, start_y + 1)?;
            write!(term, "{}", style(preview).green())?;
        }

        term.move_cursor_to(start_x, start_y + 1)?;
        write!(term, "{}", self.search_text)?;

        Ok(())
    }

    pub fn handle_keys(&mut self, key: &Key, candidates: &Vec<String>) {
        let previews = self.possible_candidates(candidates);

        match key {
            Key::ArrowRight => {
                if let Some(preview_text) = self.selected_candidates(candidates) {
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
