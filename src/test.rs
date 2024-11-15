// crates.io
use reqwest::blocking::Client as BlockingClient;
use serde_json::Value;
// self
use super::{blocking::Http as blockingHttp, *};

static CLIENT: LazyLock<Client> = crate::lazy(|| Client::default());
static BLOCKING_CLIENT: LazyLock<BlockingClient> = crate::lazy(|| BlockingClient::default());

#[tokio::test]
async fn http_and_response_should_work() {
	let resp = CLIENT.get_with_retries("https://httpbin.org/get", 3, 50).await.unwrap();

	assert!(resp.clone().text().contains("httpbin.org"));
	assert_eq!(resp.json::<Value>().unwrap()["headers"]["Host"].as_str().unwrap(), "httpbin.org");

	let resp = CLIENT.post_with_retries("https://httpbin.org/post", "hello", 3, 50).await.unwrap();

	assert!(resp.clone().text().contains("https://httpbin.org/post"));
	assert_eq!(resp.json::<Value>().unwrap()["url"].as_str().unwrap(), "https://httpbin.org/post");
}

#[cfg_attr(feature = "blocking", test)]
fn blocking_http_and_response_should_work() {
	let resp = BLOCKING_CLIENT.get_with_retries("https://httpbin.org/get", 3, 50).unwrap();

	assert!(resp.clone().text().contains("httpbin.org"));
	assert_eq!(resp.json::<Value>().unwrap()["headers"]["Host"].as_str().unwrap(), "httpbin.org");

	let resp =
		BLOCKING_CLIENT.post_with_retries("https://httpbin.org/post", "hello", 3, 50).unwrap();

	assert!(resp.clone().text().contains("https://httpbin.org/post"));
	assert_eq!(resp.json::<Value>().unwrap()["url"].as_str().unwrap(), "https://httpbin.org/post");
}
