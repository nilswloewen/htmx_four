//! The [`HX-Last-Part-ID`](https://four.htmx.org/extensions/hx-multipart#hx-last-part-id) request header.

use http::HeaderName;

/// [`HX-Last-Part-ID`](https://four.htmx.org/extensions/hx-multipart#hx-last-part-id): the last completed part, sent when reconnecting.
pub const HX_LAST_PART_ID: HeaderName = HeaderName::from_static("hx-last-part-id");
