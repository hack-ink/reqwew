<div align="center">

# reqwew
### Reqwest effortless wrapper.

[![License](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Checks](https://github.com/hack-ink/reqwew/actions/workflows/checks.yml/badge.svg?branch=main)](https://github.com/hack-ink/reqwew/actions/workflows/checks.yml)
[![Release](https://github.com/hack-ink/reqwew/actions/workflows/release.yml/badge.svg)](https://github.com/hack-ink/reqwew/actions/workflows/release.yml)
[![GitHub tag (latest by date)](https://img.shields.io/github/v/tag/hack-ink/reqwew)](https://github.com/hack-ink/reqwew/tags)
[![GitHub code lines](https://tokei.rs/b1/github/hack-ink/reqwew)](https://github.com/hack-ink/reqwew)
[![GitHub last commit](https://img.shields.io/github/last-commit/hack-ink/reqwew?color=red&style=plastic)](https://github.com/hack-ink/reqwew)
</div>


## Usage
### Async
```rs
// std
use std::sync::LazyLock;
// crates.io
use reqwew::{reqwest::Client, Http, Response};
use serde_json::Value;

// Lazy static.
pub static CLIENT: LazyLock<Client> = reqwew::lazy(|| Client::default());

// Async.
let resp = CLIENT.get_with_retries("https://httpbin.org/get", 3, 50).await.unwrap();

assert!(resp.clone().text().contains("httpbin.org"));
assert_eq!(resp.json::<Value>().unwrap()["headers"]["Host"].as_str().unwrap(), "httpbin.org");

let resp = CLIENT.post_with_retries("https://httpbin.org/post", "hello", 3, 50).await.unwrap();

assert!(resp.clone().text().contains("https://httpbin.org/post"));
assert_eq!(resp.json::<Value>().unwrap()["url"].as_str().unwrap(), "https://httpbin.org/post");
```

### Blocking
```rs
// std
use std::sync::LazyLock;
// crates.io
use reqwew::{
blocking::Http as BlockingHttp, reqwest::blocking::Client as BlockingClient, Response,
};
use serde_json::Value;

// Lazy static.
pub static BLOCKING_CLIENT: LazyLock<BlockingClient> = reqwew::lazy(|| BlockingClient::default());

// Blocking.
let resp = BLOCKING_CLIENT.get_with_retries("https://httpbin.org/get", 3, 50).unwrap();

assert!(resp.clone().text().contains("httpbin.org"));
assert_eq!(resp.json::<Value>().unwrap()["headers"]["Host"].as_str().unwrap(), "httpbin.org");

let resp = BLOCKING_CLIENT.post_with_retries("https://httpbin.org/post", "hello", 3, 50).unwrap();

assert!(resp.clone().text().contains("https://httpbin.org/post"));
assert_eq!(resp.json::<Value>().unwrap()["url"].as_str().unwrap(), "https://httpbin.org/post");
```
