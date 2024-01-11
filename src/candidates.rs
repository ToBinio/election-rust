use std::fs;
use std::path::Path;
use std::string::ToString;

pub const FILE_PATH: &str = "candidates.txt";

pub fn load_candidates<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<String>> {
    let content = fs::read_to_string(path)?;

    Ok(content.lines().map(|name| name.to_string()).collect())
}

pub fn save_candidates<P: AsRef<Path>>(path: P, candidates: &Vec<String>) -> anyhow::Result<()> {
    let content = candidates.join("\n").to_string();

    fs::write(path, content)?;

    Ok(())
}
