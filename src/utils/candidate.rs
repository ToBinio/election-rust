use crate::voting::candidate::Candidate;
use iter_tools::all;
use std::fs;
use std::path::Path;

pub fn load_candidates<P: AsRef<Path>>(
    path: P,
    allowed_votes: usize,
) -> anyhow::Result<Vec<Candidate>> {
    let content = fs::read_to_string(path)?;

    Ok(content
        .lines()
        .map(|name| Candidate::new(name.to_string(), allowed_votes))
        .collect())
}

pub fn save_candidates<P: AsRef<Path>>(path: P, candidates: &[String]) -> anyhow::Result<()> {
    let content = candidates.join("\n").to_string();

    fs::write(path, content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::utils::candidate::{load_candidates, save_candidates};
    use iter_tools::Itertools;
    use std::env::temp_dir;
    use std::fs;

    #[test]
    fn load_candidate_returns_correct() {
        let temp_path = temp_dir();

        let candidate_path = temp_path.join("candidates.txt");

        let _ = fs::write(&candidate_path, "huff\npuff\nmuff");

        let candidates = load_candidates(&candidate_path, 2).unwrap();

        let candidate_names = candidates
            .iter()
            .map(|candidate| candidate.name.clone())
            .collect_vec();

        assert_eq!(
            candidate_names,
            vec!["huff".to_string(), "puff".to_string(), "muff".to_string()]
        )
    }

    #[test]
    fn save_candidate_returns_correct() {
        let temp_path = temp_dir();

        let candidate_path = temp_path.join("candidates.txt");

        save_candidates(
            &candidate_path,
            &["huff".to_string(), "puff".to_string(), "muff".to_string()],
        )
        .unwrap();

        let content = fs::read_to_string(candidate_path).unwrap();

        assert_eq!(content, "huff\npuff\nmuff")
    }
}
