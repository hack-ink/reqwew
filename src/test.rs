// crates.io
#[cfg(feature = "reqwest")] use reqwest::Method;
#[cfg(feature = "blocking")] use reqwest::blocking::Client as BlockingClient;
#[cfg(feature = "json")] use serde_json::Value;
// self
use super::*;
#[cfg(feature = "blocking")] use blocking::Http as blockingHttp;

#[cfg(feature = "reqwest")]
static CLIENT: LazyLock<Client> = crate::lazy(Client::default);
#[cfg(feature = "blocking")]
static BLOCKING_CLIENT: LazyLock<BlockingClient> = crate::lazy(BlockingClient::default);

#[cfg(feature = "reqwest")]
#[tokio::test]
async fn http_and_response_should_work() {
	let req = || async {
		CLIENT
			.request_with_retries(
				CLIENT.request(Method::GET, "https://httpbin.org/get").build().unwrap(),
				3,
				50,
			)
			.await
			.unwrap()
	};

	assert!(req().await.text().await.unwrap().contains("httpbin.org"));
	#[cfg(feature = "json")]
	assert_eq!(
		req().await.json::<Value>().await.unwrap()["headers"]["Host"].as_str().unwrap(),
		"httpbin.org"
	);

	let req = || async {
		CLIENT
			.request_with_retries(
				CLIENT
					.request(Method::POST, "https://httpbin.org/post")
					.body("hello")
					.build()
					.unwrap(),
				3,
				50,
			)
			.await
			.unwrap()
	};

	assert!(req().await.text().await.unwrap().contains("https://httpbin.org/post"));
	#[cfg(feature = "json")]
	assert_eq!(
		req().await.json::<Value>().await.unwrap()["url"].as_str().unwrap(),
		"https://httpbin.org/post"
	);
}

#[cfg(feature = "blocking")]
#[test]
fn blocking_http_and_response_should_work() {
	let req = || {
		BLOCKING_CLIENT
			.request_with_retries(
				BLOCKING_CLIENT.request(Method::GET, "https://httpbin.org/get").build().unwrap(),
				3,
				50,
			)
			.unwrap()
	};

	assert!(req().text().unwrap().contains("httpbin.org"));
	#[cfg(feature = "json")]
	assert_eq!(req().json::<Value>().unwrap()["headers"]["Host"].as_str().unwrap(), "httpbin.org");

	let req = || {
		BLOCKING_CLIENT
			.request_with_retries(
				BLOCKING_CLIENT
					.request(Method::POST, "https://httpbin.org/post")
					.body("hello")
					.build()
					.unwrap(),
				3,
				50,
			)
			.unwrap()
	};

	assert!(req().text().unwrap().contains("https://httpbin.org/post"));
	#[cfg(feature = "json")]
	assert_eq!(req().json::<Value>().unwrap()["url"].as_str().unwrap(), "https://httpbin.org/post");
}
