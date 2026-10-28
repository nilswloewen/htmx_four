//! The [`HX-Preloaded`](https://four.htmx.org/extensions/hx-preload#usage) request header.

use http::HeaderName;

/// [`HX-Preloaded`](https://four.htmx.org/extensions/hx-preload#usage): the docs say preload requests send `true`.
///
/// The 4.0.0 extension does not send it (bigskysoftware/htmx#4110).
pub const HX_PRELOADED: HeaderName = HeaderName::from_static("hx-preloaded");
