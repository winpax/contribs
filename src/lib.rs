pub mod models;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("Invalid namespace")]
    InvalidNamespace,
    #[error("Invalid repo")]
    InvalidRepo,
    #[error("Missing namespace")]
    MissingNamespace,
    #[error("Missing repo")]
    MissingRepo,
    #[error("Invalid namespace/repo input")]
    InvalidInput,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Contributors {
    api_key: String,
    owner: String,
    repo: String,
}

impl Contributors {
    pub fn new(api_key: String, owner: String, repo: String) -> reqwest::Result<Self> {
        Ok(Self {
            api_key,
            owner,
            repo,
        })
    }
}

impl std::future::IntoFuture for Contributors {
    type Output = reqwest::Result<reqwest::Response>;

    type IntoFuture = futures_core::future::BoxFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        use reqwest::header::ACCEPT;

        let request = reqwest::Client::new()
            .get(format!(
                "https://api.github.com/repos/{}/{}/collaborators",
                self.owner, self.repo
            ))
            .bearer_auth(self.api_key)
            .header(ACCEPT, "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send();

        Box::pin(request)
    }
}
