<div align="center">

# reqwew

### HTTP client effortless wrapper

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Checks](https://github.com/hack-ink/reqwew/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/reqwew/actions/workflows/checks.yml)
[![Release](https://github.com/hack-ink/reqwew/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/reqwew/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/reqwew)](https://github.com/hack-ink/reqwew/tags)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/reqwew)](https://github.com/hack-ink/reqwew)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/reqwew?color=red&style=plastic)](https://github.com/hack-ink/reqwew)

At the beginning, the goal was to create an easy-to-use wrapper for [reqwest](https://github.com/seanmonstar/reqwest).

Now it has evolved into a more generic solution, allowing you to implement the `HTTP` trait for any client to enjoy the handy features provided by reqwew.
</div>

## Usage

### Async

```rs
// std
use std::sync::LazyLock;
// crates.io
use reqwew::{
	reqwest::{Client, Method},
	Http, Response,
};
use serde_json::Value;

// Lazy static.
pub static CLIENT: LazyLock<Client> = reqwew::lazy(Client::default);

// Async.
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
assert_eq!(
	req().await.json::<Value>().await.unwrap()["headers"]["Host"].as_str().unwrap(),
	"httpbin.org"
);

let req = || async {
	CLIENT
		.request_with_retries(
			CLIENT.request(Method::POST, "https://httpbin.org/post").body("hello").build().unwrap(),
			3,
			50,
		)
		.await
		.unwrap()
};

assert!(req().await.text().await.unwrap().contains("https://httpbin.org/post"));
assert_eq!(
	req().await.json::<Value>().await.unwrap()["url"].as_str().unwrap(),
	"https://httpbin.org/post"
);
```

### Blocking

```rs
// std
use std::sync::LazyLock;
// crates.io
use reqwew::{
	blocking::Http as BlockingHttp,
	reqwest::{blocking::Client as BlockingClient, Method},
	Response,
};
use serde_json::Value;

// Lazy static.
pub static BLOCKING_CLIENT: LazyLock<BlockingClient> = reqwew::lazy(BlockingClient::default);

// Blocking.
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
assert_eq!(req().json::<Value>().unwrap()["url"].as_str().unwrap(), "https://httpbin.org/post");
```
