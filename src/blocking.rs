//! Blocking APIs.

// std
use std::thread;
// crates.io
#[cfg(feature = "reqwest")] use reqwest::blocking::{Client, Request, Response};
// self
use crate::*;

/// Blocking HTTP client functionality.
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
	fn request(&self, request: Self::Request) -> Result<Self::Response>;

	/// Perform a generic request with retries.
	fn request_with_retries(
		&self,
		request: Self::Request,
		retries: u32,
		retry_delay_ms: u64,
	) -> Result<Self::Response> {
		for i in 1..=retries {
			match self.request(request.try_clone().ok_or(Error::NonRetriableRequest)?) {
				Ok(r) => return Ok(r),
				Err(e) => {
					tracing::error!("attempt {i}/{retries}, {e:?}, retrying in {retry_delay_ms}ms");
					thread::sleep(Duration::from_millis(retry_delay_ms));
				},
			}
		}

		Err(Error::ExceededMaxRetries(retries))?
	}
}
#[cfg(feature = "reqwest")]
impl Http for Client {
	type Request = Request;
	type Response = Response;

	fn request(&self, request: Self::Request) -> Result<Self::Response> {
		Ok(self.execute(request)?)
	}
}

#[cfg(feature = "reqwest")]
impl TryClone for Request {
	fn try_clone(&self) -> Option<Self> {
		self.try_clone()
	}
}
