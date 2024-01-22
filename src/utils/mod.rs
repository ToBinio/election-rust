use crate::utils::candidate::load_candidates;
use crate::voting::Voting;
use anyhow::{anyhow, bail};
use std::fs;
use std::path::Path;

pub mod candidate;

pub fn get_fitting_names(names: Vec<String>, search: &str) -> Vec<String> {
    names
        .iter()
        .filter(|name| name.to_uppercase().starts_with(&search.to_uppercase()))
        .map(|name| name.to_string())
        .collect()
}

pub fn elapsed_text(text: &str, max_length: usize) -> String {
    if text.len() > max_length {
        format!("{}...", &text[..(max_length - 3)])
    } else {
        text.to_string()
    }
}

pub fn load_voting<P1: AsRef<Path>, P2: AsRef<Path>>(
    candidate: P1,
    save: P2,
    allowed_votes: Option<usize>,
) -> anyhow::Result<Voting> {
    if let Ok(content) = fs::read_to_string(&save) {
        let voting = Voting::load(content)
            .map_err(|_| anyhow!("Invalid save file - {:?} ", save.as_ref()))?;

        if allowed_votes.is_some() && voting.allowed_votes != allowed_votes.unwrap() {
            bail!("you allowed votes have changed")
        }

        Ok(voting)
    } else {
        let allowed_votes = allowed_votes.unwrap_or(2);

        match load_candidates(&candidate, allowed_votes) {
            Ok(candidates) => Ok(Voting::new(candidates, save, allowed_votes)?),
            Err(_) => bail!(
                "could not file {:?} or {:?}",
                candidate.as_ref(),
                save.as_ref()
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{elapsed_text, get_fitting_names, load_voting};
    use iter_tools::Itertools;
    use std::env::temp_dir;
    use std::fs;

    #[test]
    fn get_fitting_names_returns_correct() {
        let names = vec![
            "Huff".to_string(),
            "Frischmann".to_string(),
            "Jonas".to_string(),
            "huff huff".to_string(),
            "muff".to_string(),
        ];

        let fitting_names = get_fitting_names(names, "hu");
        assert_eq!(fitting_names, vec!["Huff", "huff huff"]);
    }

    #[test]
    fn get_fitting_names_handle_empty() {
        let names = vec![
            "Huff".to_string(),
            "Frischmann".to_string(),
            "Jonas".to_string(),
            "huff huff".to_string(),
            "muff".to_string(),
        ];

        let fitting_names = get_fitting_names(names.clone(), "");
        assert_eq!(fitting_names, names);
    }

    #[test]
    fn get_fitting_names_handles_not_fit() {
        let names = vec![
            "Huff".to_string(),
            "Frischmann".to_string(),
            "Jonas".to_string(),
            "huff huff".to_string(),
            "muff".to_string(),
        ];

        let fitting_names = get_fitting_names(names.clone(), "tobi");
        assert_eq!(fitting_names, Vec::<String>::new());
    }

    #[test]
    fn elapsed_text_returns_correct() {
        assert_eq!(elapsed_text("huff", 10), "huff".to_string());
        assert_eq!(elapsed_text("huff", 3), "...".to_string());
        assert_eq!(elapsed_text("huff huff", 4), "h...".to_string());
    }

    #[test]
    fn load_vote_from_candidates() {
        let temp_path = temp_dir();

        let candidate_path = temp_path.join("candidates.txt");

        let _ = fs::write(&candidate_path, "huff\npuff\nmuff");

        let voting = load_voting(&candidate_path, "", Some(2)).unwrap();

        let candidate_names = voting
            .candidates
            .iter()
            .map(|candidate| candidate.name.clone())
            .collect_vec();

        assert_eq!(
            candidate_names,
            vec!["huff".to_string(), "puff".to_string(), "muff".to_string()]
        )
    }

    #[test]
    fn load_vote_from_save() {
        let temp_path = temp_dir();

        let save_path = temp_path.join("save.txt");

        let _ = fs::write(
            &save_path,
            r#"{"candidate_selections":[{"search_text":"","selected_preview":0,"header":"First"},{"search_text":"","selected_preview":0,"header":"Second"}],"candidates":[{"name":"huff","votes":[0,0]}],"papers":[],"invalid_vote_count":1,"allowed_votes":2,"save_path":"save.json"}"#,
        );

        let voting = load_voting("", &save_path, Some(2)).unwrap();

        let candidate_names = voting
            .candidates
            .iter()
            .map(|candidate| candidate.name.clone())
            .collect_vec();

        assert_eq!(candidate_names, vec!["huff".to_string()])
    }
}
