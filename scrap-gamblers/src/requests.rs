use bevy::tasks::{AsyncComputeTaskPool, Task};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

// ── Low-level async functions ─────────────────────────────────────────────────

/// Fetches a URL and returns the response body as a plain string.
pub async fn get_text(url: &str) -> Result<String, reqwest::Error> {
    Client::new().get(url).send().await?.text().await
}

/// Fetches a URL and deserialises the JSON response body into `T`.
pub async fn get_json<T: DeserializeOwned>(url: &str) -> Result<T, reqwest::Error> {
    Client::new().get(url).send().await?.json::<T>().await
}

/// POSTs `body` serialised as JSON and deserialises the response into `R`.
pub async fn post_json<B, R>(url: &str, body: &B) -> Result<R, reqwest::Error>
where
    B: Serialize + ?Sized,
    R: DeserializeOwned,
{
    Client::new()
        .post(url)
        .json(body)
        .send()
        .await?
        .json::<R>()
        .await
}

// ── Bevy task helpers ─────────────────────────────────────────────────────────
//
// Spawn a request on Bevy's AsyncComputeTaskPool so it runs off the main
// thread (native) or via the browser executor (WASM).  Poll the returned
// `Task` in a system with `future::block_on(future::poll_once(&mut task))`.
//
// Example:
//   commands.spawn(fetch_text("https://example.com/api"));
//
// Then in a system:
//   fn poll_text(mut q: Query<(Entity, &mut FetchText)>, mut commands: Commands) {
//       for (e, mut task) in &mut q {
//           if let Some(result) = block_on(poll_once(&mut task.0)) {
//               info!("{result:?}");
//               commands.entity(e).despawn();
//           }
//       }
//   }

/// Component that wraps a running text-fetch task.
#[derive(bevy::prelude::Component)]
pub struct FetchText(pub Task<Result<String, reqwest::Error>>);

/// Component that wraps a running JSON-fetch task.
#[derive(bevy::prelude::Component)]
pub struct FetchJson<T: Send + 'static>(pub Task<Result<T, reqwest::Error>>);

/// Spawns a GET request and returns a `FetchText` component to attach to an entity.
pub fn fetch_text(url: impl Into<String>) -> FetchText {
    let url = url.into();
    let task = AsyncComputeTaskPool::get().spawn(async move { get_text(&url).await });
    FetchText(task)
}

/// Spawns a GET-JSON request and returns a `FetchJson<T>` component.
pub fn fetch_json<T>(url: impl Into<String>) -> FetchJson<T>
where
    T: DeserializeOwned + Send + 'static,
{
    let url = url.into();
    let task = AsyncComputeTaskPool::get().spawn(async move { get_json::<T>(&url).await });
    FetchJson(task)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    // httpbin.org is a free public HTTP testing service.
    const HTTPBIN: &str = "https://httpbin.org";

    #[derive(Debug, Deserialize)]
    struct SlideShow {
        slideshow: Slide,
    }
    #[derive(Debug, Deserialize)]
    struct Slide {
        title: String,
    }

    #[derive(Debug, Deserialize)]
    struct PostEcho {
        json: serde_json::Value,
    }

    #[tokio::test]
    async fn test_get_text() {
        let body = get_text(&format!("{HTTPBIN}/get")).await.unwrap();
        assert!(body.contains("httpbin.org"), "unexpected body: {body}");
    }

    #[tokio::test]
    async fn test_get_json() {
        let result: SlideShow = get_json(&format!("{HTTPBIN}/json")).await.unwrap();
        assert!(!result.slideshow.title.is_empty());
    }

    #[tokio::test]
    async fn test_post_json() {
        let payload = serde_json::json!({ "player": "test", "score": 42 });
        let echo: PostEcho = post_json(&format!("{HTTPBIN}/post"), &payload).await.unwrap();
        assert_eq!(echo.json["score"], 42);
    }
}
