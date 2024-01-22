use crate::utils::candidate::load_candidates;
use crate::voting::Voting;
use anyhow::{anyhow, bail};
use std::fs;
use std::path::Path;

pub mod candidate;

pub fn get_fitting_names(names: Vec<String>, search: &String) -> Vec<String> {
    names
        .iter()
        .filter(|name| name.starts_with(search))
        .map(|name| name.to_string())
        .collect()
}

pub fn elepesed_text(text: &String, max_length: usize) -> String {
    if text.len() > max_length {
        format!("{}...", &text[..(max_length - 3)])
    } else {
        text.to_string()
    }
}

pub fn load_voting<P: AsRef<Path>>(candidate: P, save: P) -> anyhow::Result<Voting> {
    if let Ok(content) = fs::read_to_string(&save) {
        Ok(Voting::load(content)
            .map_err(|_| anyhow!("Invalid save file - {:?} ", save.as_ref()))?)
    } else {
        match load_candidates(&candidate) {
            Ok(candidates) => Ok(Voting::new(candidates, save)),
            Err(_) => bail!(
                "could not file {:?} or {:?}",
                candidate.as_ref(),
                save.as_ref()
            ),
        }
    }
}
