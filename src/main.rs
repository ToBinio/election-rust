use crate::cli::{Cli, SubCommands};
use crate::terminal::candidate_display::{CandidateDisplay, CandidateDisplayState};
use crate::terminal::voting_display::{VotingDisplay, VotingDisplayState};
use crate::utils::candidate::{load_candidates, FILE_PATH};
use clap::Parser;

mod terminal;
mod utils;

mod cli;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => match load_candidates(FILE_PATH) {
            Ok(candidates) => {
                let mut display = VotingDisplay::new(candidates);
                while display.handle_input().unwrap() != VotingDisplayState::Done {}
            }
            Err(_) => {
                println!("could not file candidates.txt :8")
            }
        },
        Some(subcommand) => match subcommand {
            SubCommands::Candidates => {
                let mut display = CandidateDisplay::new();
                while display.handle_input().unwrap() != CandidateDisplayState::Done {}
            }
        },
    }
}
