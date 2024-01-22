use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BallotPaper {
    pub voting: Vec<String>,
    pub disabled: bool,
    pub invalid: bool,
}

impl BallotPaper {
    pub fn new(voting: Vec<String>, invalid: bool) -> BallotPaper {
        BallotPaper {
            voting,
            disabled: false,
            invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::voting::ballot::BallotPaper;

    #[test]
    fn constructor() {
        let paper = BallotPaper::new(vec!["test".to_string()], false);

        assert_eq!(paper.voting, vec!["test".to_string()]);
        assert_eq!(paper.invalid, false);
        assert_eq!(paper.disabled, false);
    }
}
