//! The [`Last-Event-ID`](https://four.htmx.org/extensions/hx-sse#last-event-id) request header.

header_names! {
    /// [`Last-Event-ID`](https://four.htmx.org/extensions/hx-sse#last-event-id): the last SSE id, sent when the stream reconnects.
    LAST_EVENT_ID = "last-event-id";
}
