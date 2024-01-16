use crate::utils::candidate::Candidate;

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
