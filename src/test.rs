// crates.io
#[cfg(feature = "blocking")] use reqwest::blocking::Client as BlockingClient;
use reqwest::Method;
use serde_json::Value;
// self
use super::*;
#[cfg(feature = "blocking")] use blocking::Http as blockingHttp;

static CLIENT: LazyLock<Client> = crate::lazy(|| Client::default());
#[cfg(feature = "blocking")]
static BLOCKING_CLIENT: LazyLock<BlockingClient> = crate::lazy(|| BlockingClient::default());

#[tokio::test]
async fn http_and_response_should_work() {
	let resp = CLIENT
		.request_with_retries(
			CLIENT.request(Method::GET, "https://httpbin.org/get").build().unwrap(),
			3,
			50,
		)
		.await
		.unwrap();

	assert!(resp.clone().text().contains("httpbin.org"));
	assert_eq!(resp.json::<Value>().unwrap()["headers"]["Host"].as_str().unwrap(), "httpbin.org");

	let resp = CLIENT
		.request_with_retries(
			CLIENT.request(Method::POST, "https://httpbin.org/post").body("hello").build().unwrap(),
			3,
			50,
		)
		.await
		.unwrap();

	assert!(resp.clone().text().contains("https://httpbin.org/post"));
	assert_eq!(resp.json::<Value>().unwrap()["url"].as_str().unwrap(), "https://httpbin.org/post");
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_http_and_response_should_work() {
	let resp = BLOCKING_CLIENT
		.request_with_retries(
			BLOCKING_CLIENT.request(Method::GET, "https://httpbin.org/get").build().unwrap(),
			3,
			50,
		)
		.unwrap();

	assert!(resp.clone().text().contains("httpbin.org"));
	assert_eq!(resp.json::<Value>().unwrap()["headers"]["Host"].as_str().unwrap(), "httpbin.org");

	let resp = BLOCKING_CLIENT
		.request_with_retries(
			BLOCKING_CLIENT
				.request(Method::POST, "https://httpbin.org/post")
				.body("hello")
				.build()
				.unwrap(),
			3,
			50,
		)
		.unwrap();

	assert!(resp.clone().text().contains("https://httpbin.org/post"));
	assert_eq!(resp.json::<Value>().unwrap()["url"].as_str().unwrap(), "https://httpbin.org/post");
}
