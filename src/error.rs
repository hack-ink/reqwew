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

	#[error("[api] max retries exceeded after {retries} attempts")]
	ExceededMaxRetries { retries: u32 },
}
