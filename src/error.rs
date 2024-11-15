//! Reqwew error types.

/// Reqwew result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Reqwew error type.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Reqwest(#[from] reqwest::Error),
	#[error(transparent)]
	SerdeJson(#[from] serde_json::Error),

	#[error("[reqwew] non-retriable request; this typically occurs when attempting to retry a stream body request")]
	NonRetriableRequest,
	#[error("[reqwew] max retries exceeded after {0} attempts")]
	ExceededMaxRetries(u32),
}
