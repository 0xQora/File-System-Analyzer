use clap::Parser;
use std::path::PathBuf;

use crate::display::display_directory_analyzer;
use crate::model::AnalyzeOptions;

mod analyzer;
mod display;
mod error;
mod model;

#[derive(Parser)]
#[command(author, version, about = "A File System Analyzer")]
struct Cli {
    /// Path to analyze 
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Maximum directory depth to traverse
    #[arg(short = 'd', long)]
    max_depth: Option<usize>,

    /// Number of biggest files and folders to display
    #[arg(short = 'n', long, default_value_t = 5)]
    top_n: usize,

    /// Follow symbolic links
    #[arg(short = 'L', long)]
    follow_symlinks: bool,

    /// Minimum file size to include in analysis (in bytes)
    #[arg(short = 's', long)]
    min_size: Option<u64>,

    /// Enable duplicate file detection
    #[arg(short = 'D', long)]
    duplicates: bool,

    /// Patterns to ignore ("node_modules/**", "*.tmp")
    #[arg(short = 'i', long, value_delimiter = ',')]
    ignore: Option<Vec<String>>,
}


fn main() {
    let cli = Cli::parse();

    let options = AnalyzeOptions::new(
        cli.path,
        cli.max_depth,
        cli.top_n,
        cli.follow_symlinks,
        cli.min_size,
        cli.duplicates,
        cli.ignore.unwrap_or_default(),
    );

    if let Err(e) = display_directory_analyzer(options) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

