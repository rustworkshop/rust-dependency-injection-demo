use crate::shared::ReaderError;
use ::function_name::named;
use std::future::Future;
use std::process::exit;

mod boxed_trait_struct_injection;
mod generic_trait_struct_injection;
mod no_injection;
mod shared;

// main cli that uses the business logic and will need to inject the real dependency
#[tokio::main] // tokyo gives us async https://docs.rs/tokio/latest/tokio/index.html
async fn main() {
    run(run_no_injection).await;
    run(run_boxed_trait_struct_injection).await;
    run(run_boxed_trait_struct_injection_with_new).await;
    run(run_generic_trait_struct_injection).await;
    run(run_generic_trait_struct_injection_with_new).await;
}

async fn run<T: Future<Output = Result<String, ReaderError>>>(p0: fn() -> T) {
    match p0().await {
        Ok(output) => {
            println!("{}", output);
        }
        Err(err) => {
            eprintln!("failed: {}", err);
            exit(1);
        }
    };
}

#[named]
async fn run_no_injection() -> Result<String, ReaderError> {
    print!("{}: ", function_name!());
    // set up business logic with injected dependency(s)
    let analyser = no_injection::RepoAnalyser {};
    analyser.analyse_repo("timabell/gitopolis").await // run the business logic
}

#[named]
async fn run_boxed_trait_struct_injection() -> Result<String, ReaderError> {
    print!("{}: ", function_name!());
    // set up business logic with injected dependency(s)
    let analyser = boxed_trait_struct_injection::RepoAnalyser {
        repo_reader: Box::new(boxed_trait_struct_injection::RealRepoReader {}), // inject the real implementation of RepoReader in to the analyser
    };
    analyser.analyse_repo("timabell/gitopolis").await // run the business logic
}

#[named]
async fn run_boxed_trait_struct_injection_with_new() -> Result<String, ReaderError> {
    print!("{}: ", function_name!());
    // set up business logic with injected dependency(s)
    let analyser = boxed_trait_struct_injection::RepoAnalyser::new(Box::new(
        boxed_trait_struct_injection::RealRepoReader {},
    )); // inject the real implementation of RepoReader in to the analyser
    analyser.analyse_repo("timabell/schema-explorer").await // run the business logic
}

#[named]
async fn run_generic_trait_struct_injection() -> Result<String, ReaderError> {
    print!("{}: ", function_name!());
    // set up business logic with injected dependency(s)
    let analyser = generic_trait_struct_injection::RepoAnalyser {
        repo_reader: generic_trait_struct_injection::RealRepoReader {}, // inject the real implementation of RepoReader in to the analyser
    };
    analyser.analyse_repo("timabell/gitopolis").await // run the business logic
}

#[named]
async fn run_generic_trait_struct_injection_with_new() -> Result<String, ReaderError> {
    print!("{}: ", function_name!());
    // set up business logic with injected dependency(s)
    let analyser = generic_trait_struct_injection::RepoAnalyser::new(
        generic_trait_struct_injection::RealRepoReader {},
    ); // inject the real implementation of RepoReader in to the analyser
    analyser.analyse_repo("timabell/schema-explorer").await // run the business logic
}
