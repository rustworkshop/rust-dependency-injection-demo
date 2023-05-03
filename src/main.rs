use crate::async_trait_injection::{RealRepoReader, RepoAnalyser};
use reqwest::Error; // todo: this is leaking from the injected dependency into the compilation of main, it shouldn't imho

mod async_trait_injection;

// main cli that uses the business logic and will need to inject the real dependency
#[tokio::main] // tokyo gives us async https://docs.rs/tokio/latest/tokio/index.html
async fn main() -> Result<(), Error> {
    run_with_async_trait().await?;
    Ok(())
}

async fn run_with_async_trait() -> Result<(), Error> {
    // set up business logic with injected dependency(s)
    let analyser = RepoAnalyser {
        repo_reader: Box::new(RealRepoReader {}), // inject the real implementation of RepoReader in to the analyser
    };

    let repo_analysis = analyser.analyse_repo("timabell/gitopolis").await?; // run the business logic
    println!("{}", repo_analysis);
    Ok(())
}
