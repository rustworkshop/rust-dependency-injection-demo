use async_trait::async_trait;
use serde::Deserialize;
use std::fmt;
use std::fmt::Formatter;

// This file demonstrates doing dependency injection with (async) traits, and hiding the inner Error
// types that the dependency handles.

// RepoAnalyser is the "business logic" under test. It depends on the trait instead of a real implementation.

// RepoReader is the trait that defines what it needs injected. This is implemented by:
// - RealRepoReader actually talks to github
// - FakeRepoReader responds with a predetermined star count for testing the RepoAnalyser logic

#[derive(Deserialize, Debug)]
struct Repo {
    stargazers_count: u32,
}

// Wrapper for the various types of error the dependency can encounter so that the
// "business logic (RepoAnalyser)" doesn't need to know about any inner error types.
#[derive(Debug)]
pub enum ReaderError {
    ReqwestError(reqwest::Error),
    JsonError(serde_json::Error),
}

// Implement display so that calling code doesn't need to know the individual error types when displaying failures
impl fmt::Display for ReaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReaderError::ReqwestError(e) => write!(f, "{}", e),
            ReaderError::JsonError(e) => write!(f, "{}", e),
        }
    }
}

// business logic struct
pub struct RepoAnalyser {
    pub repo_reader: Box<dyn RepoReader>, // here is the pointer to whatever was injected
}

// implementation of business logic, that has a dependency to inject
impl RepoAnalyser {
    pub async fn analyse_repo(&self, repo: &str) -> Result<String, ReaderError> {
        let stars = &self
            .repo_reader
            .get_stars(repo) // <== use the injected dependency
            .await?;
        let repo_analysis = format!("{} has {} stars", repo, stars); // some "business logic"
        Ok(repo_analysis)
    }
}

// the dependency, defined as a trait saying what we need it to do. Like an interface in C#.
#[async_trait]
pub trait RepoReader {
    async fn get_stars(&self, repo: &str) -> Result<u32, ReaderError>; // we need something that does this
}

// Empty struct to hang real logic off. Could add state/data here later if needed
pub struct RealRepoReader {}

// Real implementation of dependency logic that calls out to internet
#[async_trait]
impl RepoReader for RealRepoReader {
    async fn get_stars(&self, repo: &str) -> Result<u32, ReaderError> {
        get_stars_async(repo).await
    }
}

async fn get_stars_async(repo: &str) -> Result<u32, ReaderError> {
    // based on https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html

    // https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html#method.user_agent
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT) // needed because otherwise github starts rejecting the requests
        .build()
        .map_err(|e| ReaderError::ReqwestError(e))?;

    static API_ROOT: &str = "https://api.github.com";
    let request_url = format!("{root}/repos/{repo}", root = API_ROOT, repo = repo);
    let response = client
        .get(&request_url)
        .send()
        .await
        .map_err(|e| ReaderError::ReqwestError(e))?;
    let json_string = response
        .text()
        .await
        .map_err(|e| ReaderError::ReqwestError(e))?;
    let repo: Repo =
        serde_json::from_str(json_string.as_str()).map_err(|e| ReaderError::JsonError(e))?;
    Ok(repo.stargazers_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Empty struct to hang fake logic off. Could add state/data here later if needed
    struct FakeRepoReader {
        star_count: u32,
    }

    // Fake implementation of dependency logic that calls out to internet
    #[async_trait]
    impl RepoReader for FakeRepoReader {
        async fn get_stars(&self, _repo: &str) -> Result<u32, ReaderError> {
            Ok(self.star_count)
        }
    }
    #[tokio::test]
    async fn formats_repo_name_and_stars() {
        // set up business logic to use fake instead of talking to github
        let analyser = RepoAnalyser {
            repo_reader: Box::new(FakeRepoReader { star_count: 1000 }), // inject a fake dependency that is under the control of the tests
        };

        const REPO_PATH: &str = "timabell/gitopolis";
        let repo_analysis = analyser
            .analyse_repo(REPO_PATH)
            .await
            .expect("analysis failed"); // run the business logic
        assert_eq!(repo_analysis, "timabell/gitopolis has 1000 stars");
    }
}
