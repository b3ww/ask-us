use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("serenity error: {0}")]
    Serenity(#[from] serenity::Error),

    #[error("serenity error: {0}")]
    Sqlx(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
