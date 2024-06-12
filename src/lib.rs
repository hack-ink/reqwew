//! Reqwest effortless wrapper.

#![deny(clippy::all, missing_docs, unused_crate_dependencies)]

mod error;
use error::*;

#[cfg(test)] mod test;

pub use once_cell;
pub use reqwest;

// std
use std::{future::Future, time::Duration};
// crates.io
use bytes::Bytes;
use once_cell::sync::Lazy;
use reqwest::{Body, Client as RClient, IntoUrl};
use serde::de::DeserializeOwned;
use tokio::time;

/// Basic HTTP client functionality.
pub trait Http
where
	Self: Send + Sync,
{
	/// Perform a GET request.
	fn get<U>(&self, uri: U) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl;

	/// Perform a GET request with retries.
	fn get_with_reties<U>(
		&self,
		uri: U,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		async move {
			let u = uri.as_str();

			for i in 1..=retries {
				match self.get(u).await {
					Ok(r) => return Ok(r),
					Err(e) => {
						tracing::error!(
							"attempt {i}/{retries} failed for {u}: {e:?}, \
								retrying in {retry_delay_ms}ms"
						);
						time::sleep(Duration::from_millis(retry_delay_ms)).await;
					},
				}
			}

			Err(Error::ExceededMaxRetries { retries })?
		}
	}

	/// Perform a POST request.
	fn post<U, B>(&self, uri: U, body: B) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
		B: Send + Into<Body>;

	/// Perform a POST request with retries.
	fn post_with_retries<U, B>(
		&self,
		uri: U,
		body: B,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
		B: Send + Clone + Into<Body>,
	{
		async move {
			let u = uri.as_str();

			for i in 1..=retries {
				match self.post(u, body.clone()).await {
					Ok(r) => return Ok(r),
					Err(e) => {
						tracing::error!(
							"attempt {i}/{retries} failed for {u}: {e:?}, \
							retrying in {retry_delay_ms}ms"
						);
						time::sleep(Duration::from_millis(retry_delay_ms)).await;
					},
				}
			}

			Err(Error::ExceededMaxRetries { retries })?
		}
	}
}

/// [`reqwest::Response`] wrapper.
pub trait Response
where
	Self: AsRef<[u8]>,
{
	/// Deserialize the response into a JSON object.
	fn json<D>(&self) -> Result<D>
	where
		D: DeserializeOwned,
	{
		let s = self.as_ref();

		match serde_json::from_slice(s) {
			Ok(d) => Ok(d),
			Err(e) => {
				tracing::error!(
					"failed to deserialize the following response into an object\n{}",
					String::from_utf8_lossy(s)
				);

				Err(e)?
			},
		}
	}

	/// Convert the response into a string.
	fn text(&self) -> String {
		String::from_utf8_lossy(self.as_ref()).into()
	}
}
impl Response for Bytes {}

/// [`reqwest::Client`] wrapper.
#[derive(Debug, Default)]
pub struct Client(pub RClient);
impl From<RClient> for Client {
	fn from(value: RClient) -> Self {
		Self(value)
	}
}
impl From<&RClient> for Client {
	fn from(value: &RClient) -> Self {
		Self(value.to_owned())
	}
}
impl Http for Client {
	fn get<U>(&self, uri: U) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		let u = uri.as_str();

		tracing::debug!("GET {u}");

		async move { Ok(self.0.get(uri).send().await?.bytes().await?) }
	}

	fn post<U, B>(&self, uri: U, body: B) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
		B: Send + Into<Body>,
	{
		let u = uri.as_str();

		tracing::debug!("POST {u}");

		async move { Ok(self.0.post(uri).body(body).send().await?.bytes().await?) }
	}
}

/// Create a new static [`Client`] instance.
///
/// This is useful to avoid allocating multiple new clients.
///
/// # Example
/// ```rust
/// pub static CLIENT: Client = reqwew::static(|| reqwest::Client::new());
/// ```
pub const fn lazy<F>(f: F) -> Lazy<Client, F> {
	Lazy::new(f)
}
