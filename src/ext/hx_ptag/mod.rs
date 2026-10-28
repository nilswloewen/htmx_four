//! [`hx-ptag`](https://four.htmx.org/extensions/hx-ptag) skips a poll when the tag has not changed.

use http::HeaderName;

/// [`HX-PTag`](https://four.htmx.org/extensions/hx-ptag) on a poll request and on the response that stores the tag.
pub const HX_PTAG: HeaderName = HeaderName::from_static("hx-ptag");

pub mod request_headers;
pub mod response_headers;
