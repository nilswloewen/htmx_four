//! The [`Last-Event-ID`](https://four.htmx.org/extensions/hx-sse#last-event-id) request header.

use http::HeaderName;

/// [`Last-Event-ID`](https://four.htmx.org/extensions/hx-sse#last-event-id): the last SSE id, sent when the stream reconnects.
pub const LAST_EVENT_ID: HeaderName = HeaderName::from_static("last-event-id");
