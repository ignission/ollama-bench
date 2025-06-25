mod benchmark;
mod cli;
mod config;
mod error;
mod ollama;
mod output;
mod progress;
mod runner;
mod types;

use clap::Parser;
use std::process;

use crate::cli::Cli;
use crate::runner::BenchmarkRunner;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    let runner = BenchmarkRunner::new(cli);
    
    if let Err(e) = runner.run().await {
        eprintln!("{}", e);
        process::exit(1);
    }
}
