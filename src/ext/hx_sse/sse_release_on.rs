use serde::Serialize;

/// A value of [`sse.releaseOn`](https://four.htmx.org/extensions/hx-sse#ssereleaseon).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SseReleaseOn {
    /// `"immediate"`: release when the stream takes over.
    Immediate,
    /// `"first"`: release after the first message swaps.
    First,
    /// `"end"`: release when the stream closes.
    End,
}
