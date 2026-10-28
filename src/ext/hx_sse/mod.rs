//! [`hx-sse`](https://four.htmx.org/extensions/hx-sse) streams HTML with `text/event-stream`.

pub mod request_headers;
mod sse_config;
mod sse_release_on;

pub use sse_config::SseConfig;
pub use sse_release_on::SseReleaseOn;
