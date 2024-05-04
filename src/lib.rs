use models::Collaborators;
use reqwest::header::USER_AGENT;

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
    type Output = reqwest::Result<Collaborators>;

    type IntoFuture = futures_core::future::BoxFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        use reqwest::header::ACCEPT;

        let owner = self.owner.clone();
        let repo = self.repo.clone();

        let request = || async move {
            let response = reqwest::Client::new()
                .get(format!(
                    "https://api.github.com/repos/{owner}/{repo}/collaborators",
                ))
                .bearer_auth(self.api_key)
                .header(
                    USER_AGENT,
                    "contribs library for Rust. Finds contributors to a GitHub repository.",
                )
                .header(ACCEPT, "application/vnd.github+json")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send()
                .await?;

            response.json().await
        };

        Box::pin(request())
    }
}
