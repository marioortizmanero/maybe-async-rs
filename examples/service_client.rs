#![allow(dead_code, unused_variables)]

type Response = String;
type Url = &'static str;
type Method = String;

/// To use `maybe-async`, we must know which block of codes is only used on
/// blocking implementation, and which on async. These two implementation should
/// share the same API except for async/await keywords.
///
/// This will generate two traits: `InnerClientSync` and `InnerClientAsync`
#[maybe_async::both]
trait InnerClient {
    async fn request(method: Method, url: Url, data: String) -> Response;

    #[inline]
    async fn post(url: Url, data: String) -> Response {
        Self::request(String::from("post"), url, data).await
    }

    #[inline]
    async fn delete(url: Url, data: String) -> Response {
        Self::request(String::from("delete"), url, data).await
    }
}

/// This will generate a `ServiceClientSync`, which will implement
/// `InnerClientSync`, and a `ServiceClientAsync`, which will implement
/// `InnerClientAsync`.
///
/// If we had a single `ServiceClient` which implemented both `InnerClientSync`
/// and `InnerClientAsync`, calls to methods like `request` would be ambiguous
/// when both async and sync were enabled.
#[maybe_async::both]
pub struct ServiceClient;

/// Synchronous  implementation.
#[cfg(feature = "is_sync")]
impl InnerClientSync for ServiceClientSync {
    fn request(method: Method, url: Url, data: String) -> Response {
        // your implementation for sync, like use `reqwest::blocking` to send
        // the request
        String::from("pretend we have a response")
    }
}

/// Asynchronous implementation only.
#[cfg(feature = "is_async")]
#[async_trait::async_trait]
impl InnerClientAsync for ServiceClientAsync {
    async fn request(method: Method, url: Url, data: String) -> Response {
        // your implementation for async, like use `reqwest::client` or
        // `async_std` to send the request
        String::from("pretend we have a response")
    }
}

/// Code of upstream API are almost the same for sync and async, except for
/// async/await keyword. This will generate the same `impl` but for both
/// `ServiceClientAsync` and `ServiceClientSync`.
#[maybe_async::both]
impl ServiceClient {
    async fn create_bucket(name: String) -> Response {
        Self::post("http://correct_url4create", String::from("my_bucket")).await
        // When `is_sync` is toggle on, this block will compiles to:
        // Self::post("http://correct_url4create", String::from("my_bucket"))
    }

    async fn delete_bucket(name: String) -> Response {
        Self::delete("http://correct_url4delete", String::from("my_bucket")).await
    }
    // and another thousands of functions that interact with service side
}

#[cfg(feature = "is_sync")]
fn run_sync() {
    println!("sync impl running");
    let _ = ServiceClientSync::create_bucket("bucket".to_owned());
}

#[cfg(feature = "is_async")]
async fn run_async() {
    println!("async impl running");
    let _ = ServiceClientAsync::create_bucket("bucket".to_owned()).await;
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "is_sync")]
    run_sync();

    #[cfg(feature = "is_async")]
    run_async().await;
}
