use clap::{Parser, Subcommand};
use glob::Pattern;
use std::path::PathBuf;

mod analyzer;
mod display;
mod error;
mod model;
mod search;
mod utils;

use crate::model::{AnalyzeOptions, SearchOptions};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A File System Analyzer & Finder",
    subcommand_negates_reqs = true
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Analyze(AnalyzeCommand),
    Search(SearchCommand),
}

#[derive(clap::Args)]
struct AnalyzeCommand {
    #[arg(default_value = ".")]
    path: PathBuf,
    #[arg(short = 'd', long)]
    max_depth: Option<usize>,
    #[arg(short = 'n', long, default_value_t = 5)]
    top_n: usize,
    #[arg(short = 'L', long)]
    follow_symlinks: bool,
    #[arg(short = 's', long)]
    min_size: Option<String>,
    #[arg(short = 'D', long)]
    duplicates: bool,
    #[arg(short = 'i', long, value_delimiter = ',')]
    ignore: Option<Vec<String>>,
}

#[derive(clap::Args)]
struct SearchCommand {
    #[arg(default_value = ".")]
    path: PathBuf,
    #[arg(short = 'N', long, value_delimiter = ',')]
    name_pattern: Vec<String>,
    #[arg(short = 'c', long)]
    content_pattern: Option<String>,
    #[arg(short = 'a', long)]
    modified_after: Option<String>,
    #[arg(short = 'b', long)]
    modified_before: Option<String>,
    #[arg(long, alias = "min")]
    min_size: Option<u64>,
    #[arg(long, alias = "max")]
    max_size: Option<u64>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Analyze(analyze_cmd)) => handle_analyze(analyze_cmd),
        Some(Commands::Search(search_cmd)) => handle_search(search_cmd),
        None => handle_legacy_analyze(),
    }
}

fn handle_analyze(cmd: AnalyzeCommand) {
    match convert_analyze_command(cmd) {
        Ok(options) => {
            if let Err(e) = display::display_directory_analyzer(options) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_search(cmd: SearchCommand) {
    match convert_search_command(cmd) {
        Ok(options) => {
            if let Err(e) = display::display_search_result(options) {
                eprintln!("Search error: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_legacy_analyze() {
    let cmd = AnalyzeCommand {
        path: PathBuf::from("."),
        max_depth: None,
        top_n: 5,
        follow_symlinks: false,
        min_size: None,
        duplicates: false,
        ignore: None,
    };
    handle_analyze(cmd)
}

fn convert_analyze_command(cmd: AnalyzeCommand) -> Result<AnalyzeOptions, String> {
    AnalyzeOptions::new(
        cmd.path,
        cmd.max_depth,
        cmd.top_n,
        cmd.follow_symlinks,
        cmd.min_size,
        cmd.duplicates,
        cmd.ignore.unwrap_or_default(),
    )
}
fn convert_search_command(cmd: SearchCommand) -> Result<SearchOptions, String> {
    let name_pattern = cmd
        .name_pattern
        .into_iter()
        .map(|s| Pattern::new(&s).map_err(|e| format!("Invalid name pattern '{}': {}", s, e)))
        .collect::<Result<Vec<_>, _>>()?;

    let modified_after = utils::parse_date(cmd.modified_after, "modified_after")?;
    let modified_before = utils::parse_date(cmd.modified_before, "modified_before")?;

    SearchOptions::new(
        cmd.path,
        name_pattern,
        cmd.content_pattern,
        modified_after,
        modified_before,
        cmd.min_size,
        cmd.max_size,
    )
}
