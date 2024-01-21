use crate::voting::Voting;
use anyhow::anyhow;
use console::{style, Key, Term};
use std::io::Write;

pub struct CandidateSelectionDisplay {
    pub current_index: usize,
}

impl CandidateSelectionDisplay {
    pub fn new() -> CandidateSelectionDisplay {
        CandidateSelectionDisplay { current_index: 0 }
    }

    pub fn current_search_width(&self, voting: &Voting) -> usize {
        voting.candidate_selections[self.current_index]
            .search_text
            .len()
    }

    pub fn display(
        &self,
        term: &mut Term,
        start_x: usize,
        _width: usize,
        voting: &Voting,
    ) -> anyhow::Result<()> {
        for (index, candidate_selection) in voting.candidate_selections.iter().enumerate() {
            let is_valid = candidate_selection.is_valid(
                &voting.candidate_selections,
                &voting.candidates,
                index,
            );

            let y = index * 2;
            let preview = candidate_selection
                .selected_candidate(&voting.candidates)
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
        if self.is_on_done(voting) {
            term.move_cursor_to(25, 2 * voting.allowed_votes)?;

            write!(term, "{}", style("Done").on_yellow().bold())?;
        } else {
            term.move_cursor_to(25, 2 * voting.allowed_votes)?;
            write!(term, "{}", style("Done").yellow().bold())?;

            let selected_selection = voting
                .candidate_selections
                .get(self.current_index)
                .ok_or(anyhow!("invalid selection index"))?;

            term.move_cursor_to(
                25 + selected_selection.search_text.len(),
                self.current_index * 2 + 1,
            )?;
        }

        Ok(())
    }

    pub fn is_on_done(&self, voting: &Voting) -> bool {
        self.current_index == voting.candidate_selections.len()
    }

    pub fn handle_keys(&mut self, key: &Key, voting: &mut Voting) -> anyhow::Result<()> {
        match (key, self.is_on_done(voting)) {
            (Key::Enter, _) => {
                self.current_index += 1;
                self.current_index %= voting.candidate_selections.len() + 1;
            }
            (Key::Char(' '), true) => {
                voting.vote();

                voting.clear_selections();
            }
            (key, false) => {
                let selection = voting
                    .candidate_selections
                    .get_mut(self.current_index)
                    .unwrap();

                let previews = selection.possible_candidates_names(&voting.candidates);

                match key {
                    Key::Tab | Key::Char('\t') => {
                        selection.selected_preview += 1;
                        selection.selected_preview %= previews.len();
                    }
                    Key::Backspace => {
                        selection.search_text.pop();
                    }
                    Key::Char(char) => {
                        selection.search_text += &char.to_string();

                        if previews.is_empty() {
                            selection.selected_preview = 0;
                        } else {
                            selection.selected_preview %= previews.len();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        Ok(())
    }
}
