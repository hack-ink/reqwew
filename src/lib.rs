//! HTTP client effortless wrapper.

#![deny(clippy::all, missing_docs, unused_crate_dependencies)]

#[cfg(feature = "blocking")] pub mod blocking;

pub mod error;
use error::*;

#[cfg(test)] mod test;

#[cfg(feature = "reqwest")] pub use reqwest;

// std
use std::{future::Future, sync::LazyLock, time::Duration};
// crates.io
#[cfg(feature = "reqwest")] use reqwest::{Client, Request, Response};
use tokio::time;

/// HTTP client functionality.
pub trait Http
where
	Self: Send + Sync,
{
	/// Request type.
	///
	/// If the body is a stream type, it may not be cloneable.
	type Request: Send + TryClone;
	/// Response type.
	type Response: Send;

	/// Perform a generic request.
	fn request(
		&self,
		request: Self::Request,
	) -> impl Send + Future<Output = Result<Self::Response>>;

	/// Perform a generic request with retries.
	fn request_with_retries(
		&self,
		request: Self::Request,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Send + Future<Output = Result<Self::Response>> {
		async move {
			for i in 1..=retries {
				match self.request(request.try_clone().ok_or(Error::NonRetriableRequest)?).await {
					Ok(r) => return Ok(r),
					Err(e) => {
						tracing::error!(
							"attempt {i}/{retries}, {e:?}, retrying in {retry_delay_ms}ms"
						);
						time::sleep(Duration::from_millis(retry_delay_ms)).await;
					},
				}
			}

			Err(Error::ExceededMaxRetries(retries))?
		}
	}
}
#[cfg(feature = "reqwest")]
impl Http for Client {
	type Request = Request;
	type Response = Response;

	fn request(
		&self,
		request: Self::Request,
	) -> impl Send + Future<Output = Result<Self::Response>> {
		async move { Ok(self.execute(request).await?) }
	}
}

/// Try clone.
pub trait TryClone
where
	Self: Sized,
{
	/// Try clone the object.
	fn try_clone(&self) -> Option<Self>;
}
#[cfg(feature = "reqwest")]
impl TryClone for Request {
	fn try_clone(&self) -> Option<Self> {
		self.try_clone()
	}
}

/// Create a new lazy static client instance.
///
/// This is useful to avoid allocating multiple new clients.
///
/// # Example
/// ```rs
/// // std
/// use std::sync::LazyLock;
/// // crates.io
/// use reqwew::reqwest::{blocking::Client as BlockingClient, Client};
///
/// pub static CLIENT: LazyLock<Client> = reqwew::lazy(Client::default);
/// pub static BLOCKING_CLIENT: LazyLock<BlockingClient> = reqwew::lazy(BlockingClient::default);
/// ```
pub const fn lazy<F, C>(f: F) -> LazyLock<C, F>
where
	F: FnOnce() -> C,
{
	LazyLock::new(f)
}
