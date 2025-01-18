#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Internal Error: {0}")]
    InternalError(#[from] anyhow::Error),
}
