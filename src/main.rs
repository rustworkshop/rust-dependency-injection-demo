use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Repo {
    stargazers_count: u32,
}

#[tokio::main] // https://docs.rs/tokio/latest/tokio/index.html
async fn main() -> Result<(), Error> {
    // based on https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html

    // https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html#method.user_agent
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT) // needed because otherwise github starts rejecting the requests
        .build()?;

    const REPO_PATH: &str = "timabell/gitopolis";
    let request_url = format!("https://api.github.com/repos/{repo}", repo = REPO_PATH);
    let response = client.get(&request_url).send().await?;
    let json_string = response.text().await?;
    let repo: Repo =
        serde_json::from_str(json_string.as_str()).expect("JSON was not well-formatted");
    println!("{} has {} stars", REPO_PATH, repo.stargazers_count);
    Ok(())
}
