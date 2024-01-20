use crate::utils::candidate::Candidate;
use crate::utils::get_fitting_names;
use console::{style, Key, Term};
use std::io::Write;
use crate::voting::candidate_selection::CandidateSelection;

pub struct CandidateSelectionDisplay {}

impl CandidateSelectionDisplay {
    pub fn display(
        &self,
        candidate_selection: &CandidateSelection,
        term: &mut Term,
        start_x: usize,
        start_y: usize,
        _width: usize,
        preview: &str,
        is_valid: bool,
    ) -> anyhow::Result<()> {
        term.move_cursor_to(start_x, start_y)?;
        write!(term, "{}", style(candidate_selection.header).bold())?;

        if !is_valid {
            write!(term, " ❌")?;
        }

        term.move_cursor_to(start_x, start_y + 1)?;
        write!(term, "{}", style(preview).green())?;

        term.move_cursor_to(start_x, start_y + 1)?;
        write!(term, "{}", self.search_text)?;

        Ok(())
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
