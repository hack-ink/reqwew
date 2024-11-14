//! Reqwest effortless wrapper.

#![deny(clippy::all, missing_docs, unused_crate_dependencies)]

pub mod error;
use error::*;

#[cfg(test)] mod test;

pub use reqwest;

// std
use std::{future::Future, sync::LazyLock, time::Duration};
// crates.io
use bytes::Bytes;
use reqwest::{Body, Client as RClient, IntoUrl, Method as RMethod};
use serde::de::DeserializeOwned;
use tokio::time;

/// HTTP methods.
#[derive(Clone, Copy, Debug)]
pub enum Method {
	/// GET method.
	Get,
	/// POST method.
	Post,
	/// PUT method.
	Put,
	/// DELETE method.
	Delete,
	/// HEAD method.
	Head,
	/// OPTIONS method.
	Options,
	/// CONNECT method.
	Connect,
	/// PATCH method.
	Patch,
	/// TRACE method.
	Trace,
}
impl From<Method> for RMethod {
	fn from(method: Method) -> Self {
		match method {
			Method::Get => RMethod::GET,
			Method::Post => RMethod::POST,
			Method::Put => RMethod::PUT,
			Method::Delete => RMethod::DELETE,
			Method::Head => RMethod::HEAD,
			Method::Options => RMethod::OPTIONS,
			Method::Connect => RMethod::CONNECT,
			Method::Patch => RMethod::PATCH,
			Method::Trace => RMethod::TRACE,
		}
	}
}

/// Basic HTTP client functionality.
pub trait Http
where
	Self: Send + Sync,
{
	/// Perform a generic request.
	fn request<U, B>(
		&self,
		uri: U,
		method: Method,
		body: Option<B>,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
		B: Send + Into<Body>;

	/// Perform a generic request with retries.
	fn request_with_retries<U, B>(
		&self,
		uri: U,
		method: Method,
		body: Option<B>,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
		B: Clone + Send + Into<Body>,
	{
		async move {
			let u = uri.as_str();

			tracing::debug!("{method:?} {u}");

			for i in 1..=retries {
				match self.request(u, method, body.clone()).await {
					Ok(r) => return Ok(r),
					Err(e) => {
						tracing::error!("attempt {i}/{retries} failed for {u}: {e:?}, retrying in {retry_delay_ms}ms");
						time::sleep(Duration::from_millis(retry_delay_ms)).await;
					},
				}
			}

			Err(Error::ExceededMaxRetries(retries))?
		}
	}

	/// Perform a GET request.
	fn get<U>(&self, uri: U) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Get, None::<&[u8]>)
	}

	/// Perform a GET request with retries.
	fn get_with_retries<U>(
		&self,
		uri: U,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Get, None::<&[u8]>, retries, retry_delay_ms)
	}

	/// Perform a POST request.
	fn post<U, B>(&self, uri: U, body: B) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
		B: Send + Into<Body>,
	{
		self.request(uri, Method::Post, Some(body))
	}

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
		B: Clone + Send + Into<Body>,
	{
		self.request_with_retries(uri, Method::Post, Some(body), retries, retry_delay_ms)
	}

	/// Perform a PUT request.
	fn put<U, B>(&self, uri: U, body: B) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
		B: Send + Into<Body>,
	{
		self.request(uri, Method::Put, Some(body))
	}

	/// Perform a PUT request with retries.
	fn put_with_retries<U, B>(
		&self,
		uri: U,
		body: B,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
		B: Clone + Send + Into<Body>,
	{
		self.request_with_retries(uri, Method::Put, Some(body), retries, retry_delay_ms)
	}

	/// Perform a DELETE request.
	fn delete<U>(&self, uri: U) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Delete, None::<&[u8]>)
	}

	/// Perform a DELETE request with retries.
	fn delete_with_retries<U>(
		&self,
		uri: U,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Delete, None::<&[u8]>, retries, retry_delay_ms)
	}

	/// Perform a HEAD request.
	fn head<U>(&self, uri: U) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Head, None::<&[u8]>)
	}

	/// Perform a HEAD request with retries.
	fn head_with_retries<U>(
		&self,
		uri: U,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Head, None::<&[u8]>, retries, retry_delay_ms)
	}

	/// Perform an OPTIONS request.
	fn options<U>(&self, uri: U) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Options, None::<&[u8]>)
	}

	/// Perform an OPTIONS request with retries.
	fn options_with_retries<U>(
		&self,
		uri: U,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Options, None::<&[u8]>, retries, retry_delay_ms)
	}

	/// Perform a CONNECT request.
	fn connect<U>(&self, uri: U) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Connect, None::<&[u8]>)
	}

	/// Perform a CONNECT request with retries.
	fn connect_with_retries<U>(
		&self,
		uri: U,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Connect, None::<&[u8]>, retries, retry_delay_ms)
	}

	/// Perform a PATCH request.
	fn patch<U, B>(&self, uri: U, body: B) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
		B: Send + Into<Body>,
	{
		self.request(uri, Method::Patch, Some(body))
	}

	/// Perform a PATCH request with retries.
	fn patch_with_retries<U, B>(
		&self,
		uri: U,
		body: B,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
		B: Clone + Send + Into<Body>,
	{
		self.request_with_retries(uri, Method::Patch, Some(body), retries, retry_delay_ms)
	}

	/// Perform a TRACE request.
	fn trace<U>(&self, uri: U) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Trace, None::<&[u8]>)
	}

	/// Perform a TRACE request with retries.
	fn trace_with_retries<U>(
		&self,
		uri: U,
		retries: u32,
		retry_delay_ms: u64,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Trace, None::<&[u8]>, retries, retry_delay_ms)
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
	fn request<U, B>(
		&self,
		uri: U,
		method: Method,
		body: Option<B>,
	) -> impl Future<Output = Result<Bytes>> + Send
	where
		U: Send + IntoUrl,
		B: Send + Into<Body>,
	{
		let u = uri.as_str();

		tracing::debug!("{method:?} {u}");

		async move {
			Ok(if let Some(body) = body {
				self.0.request(method.into(), uri).body(body).send().await?.bytes().await?
			} else {
				self.0.request(method.into(), uri).send().await?.bytes().await?
			})
		}
	}
}

/// Create a new static [`Client`] instance.
///
/// This is useful to avoid allocating multiple new clients.
///
/// # Example
/// ```rust
/// use reqwew::Client;
/// use std::sync::LazyLock;
///
/// pub static CLIENT: LazyLock<Client> = reqwew::lazy(|| Client::default());
/// ```
pub const fn lazy<F>(f: F) -> LazyLock<Client, F>
where
	F: FnOnce() -> Client,
{
	LazyLock::new(f)
}
