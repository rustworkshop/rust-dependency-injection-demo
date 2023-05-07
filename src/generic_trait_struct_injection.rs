use crate::shared;
use async_trait::async_trait;

// the dependency, defined as a trait saying what we need it to do. Like an interface in C#.
#[async_trait]
pub trait RepoReader {
    async fn get_stars(&self, repo: &str) -> Result<u32, shared::ReaderError>; // we need something that does this
}

// implementation of business logic, that has a dependency to inject
pub struct RepoAnalyser<TReader: RepoReader> {
    pub repo_reader: TReader, // here whatever was injected, no dyn or box this time because we're using generics
}
impl<TReader: RepoReader> RepoAnalyser<TReader> {
    pub fn new(repo_reader: TReader) -> Self {
        RepoAnalyser { repo_reader }
    }

    pub async fn analyse_repo(&self, repo: &str) -> Result<String, shared::ReaderError> {
        let stars = &self
            .repo_reader
            .get_stars(repo) // <== use the injected dependency
            .await?;
        let repo_analysis = format!("{} has {} stars", repo, stars); // some "business logic" that needs test coverage
        Ok(repo_analysis)
    }
}

// Real implementation of dependency logic that calls out to internet
pub struct RealRepoReader {}
#[async_trait]
impl RepoReader for RealRepoReader {
    async fn get_stars(&self, repo: &str) -> Result<u32, shared::ReaderError> {
        shared::get_stars_async(repo).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Empty struct to hang fake logic off. Could add state/data here later if needed
    struct FakeRepoReader {
        star_count: u32,
    }

    // Fake implementation of dependency logic that would otherwise call out to internet
    #[async_trait]
    impl RepoReader for FakeRepoReader {
        async fn get_stars(&self, _repo: &str) -> Result<u32, shared::ReaderError> {
            Ok(self.star_count)
        }
    }
    #[tokio::test]
    async fn formats_star_info() {
        // set up business logic to use fake instead of talking to github
        let analyser = RepoAnalyser {
            repo_reader: FakeRepoReader { star_count: 1000 }, // inject a fake dependency that is under the control of the tests
        };

        const REPO_PATH: &str = "timabell/gitopolis";
        let repo_analysis = analyser
            .analyse_repo(REPO_PATH)
            .await
            .expect("analysis failed"); // run the business logic
        assert_eq!(repo_analysis, "timabell/gitopolis has 1000 stars");
    }

    #[tokio::test]
    async fn formats_star_info_with_new() {
        // set up business logic to use fake instead of talking to github
        let analyser = RepoAnalyser::new(FakeRepoReader { star_count: 1000 }); // inject a fake dependency that is under the control of the tests

        const REPO_PATH: &str = "timabell/gitopolis";
        let repo_analysis = analyser
            .analyse_repo(REPO_PATH)
            .await
            .expect("analysis failed"); // run the business logic
        assert_eq!(repo_analysis, "timabell/gitopolis has 1000 stars");
    }
}
