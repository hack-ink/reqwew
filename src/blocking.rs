//! Blocking APIs.

// std
use std::thread;
// crates.io
use reqwest::blocking::{Body, Client};
// self
use crate::*;

/// Basic blocking HTTP client functionality.
pub trait Http
where
	Self: Send + Sync,
{
	/// Perform a generic request.
	fn request<U, B>(&self, uri: U, method: Method, body: Option<B>) -> Result<Bytes>
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
	) -> Result<Bytes>
	where
		U: Send + IntoUrl,
		B: Clone + Send + Into<Body>,
	{
		let u = uri.as_str();

		tracing::debug!("{method:?} {u}");

		for i in 1..=retries {
			match self.request(u, method, body.clone()) {
				Ok(r) => return Ok(r),
				Err(e) => {
					tracing::error!("attempt {i}/{retries} failed for {u}: {e:?}, retrying in {retry_delay_ms}ms");
					thread::sleep(Duration::from_millis(retry_delay_ms));
				},
			}
		}

		Err(Error::ExceededMaxRetries(retries))?
	}

	/// Perform a GET request.
	fn get<U>(&self, uri: U) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Get, None::<&[u8]>)
	}

	/// Perform a GET request with retries.
	fn get_with_retries<U>(&self, uri: U, retries: u32, retry_delay_ms: u64) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Get, None::<&[u8]>, retries, retry_delay_ms)
	}

	/// Perform a POST request.
	fn post<U, B>(&self, uri: U, body: B) -> Result<Bytes>
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
	) -> Result<Bytes>
	where
		U: Send + IntoUrl,
		B: Clone + Send + Into<Body>,
	{
		self.request_with_retries(uri, Method::Post, Some(body), retries, retry_delay_ms)
	}

	/// Perform a PUT request.
	fn put<U, B>(&self, uri: U, body: B) -> Result<Bytes>
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
	) -> Result<Bytes>
	where
		U: Send + IntoUrl,
		B: Clone + Send + Into<Body>,
	{
		self.request_with_retries(uri, Method::Put, Some(body), retries, retry_delay_ms)
	}

	/// Perform a DELETE request.
	fn delete<U>(&self, uri: U) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Delete, None::<&[u8]>)
	}

	/// Perform a DELETE request with retries.
	fn delete_with_retries<U>(&self, uri: U, retries: u32, retry_delay_ms: u64) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Delete, None::<&[u8]>, retries, retry_delay_ms)
	}

	/// Perform a HEAD request.
	fn head<U>(&self, uri: U) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Head, None::<&[u8]>)
	}

	/// Perform a HEAD request with retries.
	fn head_with_retries<U>(&self, uri: U, retries: u32, retry_delay_ms: u64) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Head, None::<&[u8]>, retries, retry_delay_ms)
	}

	/// Perform an OPTIONS request.
	fn options<U>(&self, uri: U) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Options, None::<&[u8]>)
	}

	/// Perform an OPTIONS request with retries.
	fn options_with_retries<U>(&self, uri: U, retries: u32, retry_delay_ms: u64) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Options, None::<&[u8]>, retries, retry_delay_ms)
	}

	/// Perform a CONNECT request.
	fn connect<U>(&self, uri: U) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Connect, None::<&[u8]>)
	}

	/// Perform a CONNECT request with retries.
	fn connect_with_retries<U>(&self, uri: U, retries: u32, retry_delay_ms: u64) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Connect, None::<&[u8]>, retries, retry_delay_ms)
	}

	/// Perform a PATCH request.
	fn patch<U, B>(&self, uri: U, body: B) -> Result<Bytes>
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
	) -> Result<Bytes>
	where
		U: Send + IntoUrl,
		B: Clone + Send + Into<Body>,
	{
		self.request_with_retries(uri, Method::Patch, Some(body), retries, retry_delay_ms)
	}

	/// Perform a TRACE request.
	fn trace<U>(&self, uri: U) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request(uri, Method::Trace, None::<&[u8]>)
	}

	/// Perform a TRACE request with retries.
	fn trace_with_retries<U>(&self, uri: U, retries: u32, retry_delay_ms: u64) -> Result<Bytes>
	where
		U: Send + IntoUrl,
	{
		self.request_with_retries(uri, Method::Trace, None::<&[u8]>, retries, retry_delay_ms)
	}
}
impl Http for Client {
	fn request<U, B>(&self, uri: U, method: Method, body: Option<B>) -> Result<Bytes>
	where
		U: Send + IntoUrl,
		B: Send + Into<Body>,
	{
		let u = uri.as_str();

		tracing::debug!("{method:?} {u}");

		Ok(if let Some(body) = body {
			self.request(method.into(), uri).body(body).send()?.bytes()?
		} else {
			self.request(method.into(), uri).send()?.bytes()?
		})
	}
}
