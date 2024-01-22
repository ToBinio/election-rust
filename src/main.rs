use crate::cli::{Cli, SubCommands};
use crate::terminal::candidate_display::{CandidateDisplay, CandidateDisplayState};
use crate::terminal::result_display;
use crate::terminal::voting_display::{VotingDisplay, VotingDisplayState};
use crate::utils::load_voting;
use clap::Parser;
use std::fs;

mod terminal;
mod utils;

mod cli;

mod voting;

fn main() {
    let cli = Cli::parse();

    match run(cli) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    let save_path = cli.save_file.unwrap_or("save.json".to_string());
    let candidate_path = cli.candidate_file.unwrap_or("candidates.txt".to_string());

    match cli.command {
        None => {
            let voting = load_voting(&candidate_path, &save_path, cli.vote_count)?;
            let mut display = VotingDisplay::new(voting);
            while display.handle_input().unwrap() != VotingDisplayState::Done {}
        }
        Some(subcommand) => match subcommand {
            SubCommands::Candidates => {
                let mut display = CandidateDisplay::new(&candidate_path);
                while display.handle_input().unwrap() != CandidateDisplayState::Done {}
            }
            SubCommands::Result => {
                let voting = load_voting(&candidate_path, &save_path, cli.vote_count)?;
                result_display::display(voting);
            }
            SubCommands::Clear => {
                if fs::remove_file(&candidate_path).is_ok() {
                    println!("removed {}", &candidate_path)
                }

                if fs::remove_file(&save_path).is_ok() {
                    println!("removed {}", &save_path)
                }
            }
        },
    }

    Ok(())
}
