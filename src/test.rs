// crates.io
use serde_json::Value;
// self
use super::*;

static CLIENT: Lazy<Client> = lazy(Default::default);

#[tokio::test]
async fn http_and_response_should_work() {
	let response = CLIENT.get_with_reties("https://httpbin.org/get", 3, 500).await.unwrap();

	assert!(response.clone().text().contains("httpbin.org"));
	assert_eq!(
		response.json::<Value>().unwrap()["headers"]["Host"].as_str().unwrap(),
		"httpbin.org"
	);

	let response =
		CLIENT.post_with_retries("https://httpbin.org/post", "hello", 3, 500).await.unwrap();

	assert!(response.clone().text().contains("https://httpbin.org/post"));
	assert_eq!(
		response.json::<Value>().unwrap()["url"].as_str().unwrap(),
		"https://httpbin.org/post"
	);
}
