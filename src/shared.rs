use serde::Deserialize;
use std::fmt;
use std::fmt::Formatter;

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

// struct to deserialize the response json into
#[derive(Deserialize, Debug)]
struct Repo {
    stargazers_count: u32,
}

pub async fn get_stars_async(repo: &str) -> Result<u32, ReaderError> {
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
