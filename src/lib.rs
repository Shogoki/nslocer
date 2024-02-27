use clap::Parser;
use serde::de::value::Error;

mod analyzer;
use analyzer::{Analyzer, FileMetrics, Lang};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, num_args = 1.. , default_values_t = [".".to_string()])]
    paths: Vec<String>,
    #[arg(short, long)]
    excluded_paths: Vec<String>,
    #[arg(short, long)]
    base_path: String,
}

pub fn run(args: Args) -> Result<Vec<FileMetrics>, Error> {
    let analyzers = [
        Analyzer::new(Lang::Solidity),
        Analyzer::new(Lang::Rust),
        Analyzer::new(Lang::Javascript),
        Analyzer::new(Lang::Go),
    ];

    let metrics = analyzers
        .map(|a| a.analyze(&args.paths, &args.excluded_paths, &args.base_path))
        .into_iter()
        .flatten()
        .collect();

    Ok(metrics)
}
