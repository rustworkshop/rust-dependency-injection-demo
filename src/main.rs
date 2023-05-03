use async_trait::async_trait;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Repo {
    stargazers_count: u32,
}

// main cli that uses the business logic and will need to inject the real dependency
#[tokio::main] // https://docs.rs/tokio/latest/tokio/index.html
async fn main() -> Result<(), Error> {
    const REPO_PATH: &str = "timabell/gitopolis";

    // set up business logic
    let analyser = RepoAnalyser {
        repo_reader: Box::new(RealRepoReader {}), // inject the real implementation of RepoReader in to the analyser
    };

    let repo_analysis = analyser.analyse_repo(REPO_PATH).await?; // run the business logic

    println!("{}", repo_analysis);
    Ok(())
}

// business logic struct
struct RepoAnalyser {
    repo_reader: Box<dyn RepoReader>, // here is the pointer to whatever was injected
}

// implementation of business logic, that has a dependency to inject
impl RepoAnalyser {
    async fn analyse_repo(&self, repo: &str) -> Result<String, Error> {
        let stars = &self.repo_reader.get_stars(repo).await?; // <== use the injected dependency
        let repo_analysis = format!("{} has {} stars", repo, stars); // some "business logic"
        Ok(repo_analysis)
    }
}

// the dependency, defined as a trait saying what we need it to do. Like an interface in C#.
#[async_trait]
trait RepoReader {
    async fn get_stars(&self, repo: &str) -> Result<u32, Error>; // we need something that does this
}

// Empty struct to hang real logic off. Could add state/data here later if needed
struct RealRepoReader {}

// Real implementation of dependency logic that calls out to internet
#[async_trait]
impl RepoReader for RealRepoReader {
    async fn get_stars(&self, repo: &str) -> Result<u32, Error> {
        get_stars_async(repo).await
    }
}

async fn get_stars_async(repo: &str) -> Result<u32, Error> {
    // based on https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html

    // https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html#method.user_agent
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT) // needed because otherwise github starts rejecting the requests
        .build()?;

    static API_ROOT: &str = "https://api.github.com";
    let request_url = format!("{root}/repos/{repo}", root = API_ROOT, repo = repo);
    let response = client.get(&request_url).send().await?;
    let json_string = response.text().await?;
    let repo: Repo =
        serde_json::from_str(json_string.as_str()).expect("JSON was not well-formatted");
    Ok(repo.stargazers_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn formats_repo_name_and_stars() {
        // set up business logic
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

    // Empty struct to hang fake logic off. Could add state/data here later if needed
    struct FakeRepoReader {
        star_count: u32,
    }

    // Fake implementation of dependency logic that calls out to internet
    #[async_trait]
    impl RepoReader for FakeRepoReader {
        async fn get_stars(&self, _repo: &str) -> Result<u32, Error> {
            Ok(self.star_count)
        }
    }
}
