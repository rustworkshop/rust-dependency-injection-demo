use crate::shared;

pub struct RepoAnalyser {}
impl RepoAnalyser {
    pub async fn analyse_repo(&self, repo: &str) -> Result<String, shared::ReaderError> {
        let stars = shared::get_stars_async(repo).await?; // no injection, just call the real thing
        let repo_analysis = format!("{} has {} stars", repo, stars); // some "business logic" that needs test coverage
        Ok(repo_analysis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn formats_star_info() {
        let analyser = RepoAnalyser {};

        const REPO_PATH: &str = "timabell/gitopolis";
        let repo_analysis = analyser
            .analyse_repo(REPO_PATH)
            .await
            .expect("analysis failed"); // run the business logic
        assert_eq!(repo_analysis, "timabell/gitopolis has 9 stars"); // fragile test, result depends on network and github data
    }
}
