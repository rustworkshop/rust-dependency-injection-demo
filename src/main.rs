use crate::async_trait_injection::{ReaderError, RealRepoReader, RepoAnalyser};
use std::process::exit;

mod async_trait_injection;

// main cli that uses the business logic and will need to inject the real dependency
#[tokio::main] // tokyo gives us async https://docs.rs/tokio/latest/tokio/index.html
async fn main() {
    match run_with_async_trait().await {
        Ok(output) => {
            println!("{}", output);
        }
        Err(err) => {
            eprintln!("run_with_async_trait failed: {}", err);
            exit(1);
        }
    };
    match run_with_async_trait_with_new().await {
        Ok(output) => {
            println!("{}", output);
        }
        Err(err) => {
            eprintln!("run_with_async_trait_with_new failed: {}", err);
            exit(1);
        }
    };
}

async fn run_with_async_trait() -> Result<String, ReaderError> {
    // set up business logic with injected dependency(s)
    let analyser = RepoAnalyser {
        repo_reader: Box::new(RealRepoReader {}), // inject the real implementation of RepoReader in to the analyser
    };
    analyser.analyse_repo("timabell/gitopolis").await // run the business logic
}

async fn run_with_async_trait_with_new() -> Result<String, ReaderError> {
    // set up business logic with injected dependency(s)
    let analyser = RepoAnalyser::new(Box::new(RealRepoReader {})); // inject the real implementation of RepoReader in to the analyser
    analyser.analyse_repo("timabell/schema-explorer").await // run the business logic
}
