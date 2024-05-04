pub mod collaborators;
pub mod contributors;
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
