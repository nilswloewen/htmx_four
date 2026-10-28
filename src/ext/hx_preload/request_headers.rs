use http::HeaderName;

/// The upstream docs promise this header, but the 4.0.0 extension does not send it
/// (bigskysoftware/htmx#4110).
pub const HX_PRELOADED: HeaderName = HeaderName::from_static("hx-preloaded");
