use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Invalid session id")]
    InvalidSessionId,
}
