use crate::models::contributors;

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
    type Output = reqwest::Result<contributors::Contributors>;

    type IntoFuture = futures_core::future::BoxFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        use reqwest::header::{ACCEPT, USER_AGENT};

        let owner = self.owner.clone();
        let repo = self.repo.clone();

        let request = || async move {
            let response = reqwest::Client::new()
                .get(format!(
                    "https://api.github.com/repos/{owner}/{repo}/contributors",
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
