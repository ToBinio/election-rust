use crate::cli::{Cli, SubCommands};
use crate::terminal::candidate_display::{CandidateDisplay, CandidateDisplayState};
use crate::terminal::result_display;
use crate::terminal::voting_display::{VotingDisplay, VotingDisplayState};
use crate::utils::load_voting;
use clap::Parser;
use console::Term;
use std::fs;
use std::process::exit;

mod terminal;
mod utils;

mod cli;

mod voting;

fn main() {
    let cli = Cli::parse();

    let _ = ctrlc::set_handler(|| {
        let term = Term::stdout();
        term.clear_last_lines(term.size().0 as usize).unwrap();
        term.show_cursor().unwrap();

        exit(0);
    });

    match run(cli) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        None => {
            let voting = load_voting("candidates.txt", "save.json")?;
            let mut display = VotingDisplay::new(voting);
            while display.handle_input().unwrap() != VotingDisplayState::Done {}
        }
        Some(subcommand) => match subcommand {
            SubCommands::Candidates => {
                let mut display = CandidateDisplay::new("candidates.txt");
                while display.handle_input().unwrap() != CandidateDisplayState::Done {}
            }
            SubCommands::Result => {
                let voting = load_voting("candidates.txt", "save.json")?;
                result_display::display(voting);
            }
            SubCommands::Clear => {
                if fs::remove_file("candidates.txt").is_ok() {
                    println!("removed candidates.txt")
                }

                if fs::remove_file("save.json").is_ok() {
                    println!("removed save.json")
                }
            }
        },
    }

    Ok(())
}
