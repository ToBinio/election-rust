use clap::ColorChoice;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None,color = ColorChoice::Always)]
#[command(next_line_help = true)]
pub struct Cli {
    #[arg(short, long)]
    pub save_file: Option<String>,
    #[arg(short, long)]
    pub candidate_file: Option<String>,
    #[arg(short, long)]
    pub vote_count: Option<usize>,
    #[command(subcommand)]
    pub command: Option<SubCommands>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    Candidates,
    Result,
    Clear,
}
