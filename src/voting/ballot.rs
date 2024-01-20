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
