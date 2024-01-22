use crate::voting::candidate::Candidate;
use std::fs;
use std::path::Path;

pub fn load_candidates<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Candidate>> {
    let content = fs::read_to_string(path)?;

    Ok(content
        .lines()
        .map(|name| Candidate::new(name.to_string(), 2))
        .collect())
}

pub fn save_candidates<P: AsRef<Path>>(path: P, candidates: &[String]) -> anyhow::Result<()> {
    let content = candidates.join("\n").to_string();

    fs::write(path, content)?;

    Ok(())
}
