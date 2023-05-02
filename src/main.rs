use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Repo {
    stargazers_count: u32,
}

#[tokio::main] // https://docs.rs/tokio/latest/tokio/index.html
async fn main() -> Result<(), Error> {
    const REPO_PATH: &str = "timabell/gitopolis";

    let stars = get_stars(REPO_PATH).await?;

    println!("{} has {} stars", REPO_PATH, stars);
    Ok(())
}

async fn get_stars(repo: &str) -> Result<u32, Error> {
    // based on https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html

    // https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html#method.user_agent
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT) // needed because otherwise github starts rejecting the requests
        .build()?;

    let request_url = format!("https://api.github.com/repos/{repo}", repo = repo);
    let response = client.get(&request_url).send().await?;
    let json_string = response.text().await?;
    let repo: Repo =
        serde_json::from_str(json_string.as_str()).expect("JSON was not well-formatted");
    Ok(repo.stargazers_count)
}
