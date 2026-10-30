//! The htmx 4 request headers besides [`HX-Request`](https://four.htmx.org/reference/headers/HX-Request)
//! and [`HX-Request-Type`](https://four.htmx.org/reference/headers/HX-Request-Type).

use crate::request_headers::{
    HX_BOOSTED, HX_CURRENT_URL, HX_HISTORY_RESTORE_REQUEST, HX_SOURCE, HX_TARGET,
};

/// [`HX-Boosted`](https://four.htmx.org/reference/headers/HX-Boosted),
/// [`HX-Current-URL`](https://four.htmx.org/reference/headers/HX-Current-URL),
/// [`HX-History-Restore-Request`](https://four.htmx.org/reference/headers/HX-History-Restore-Request),
/// [`HX-Source`](https://four.htmx.org/reference/headers/HX-Source), and
/// [`HX-Target`](https://four.htmx.org/reference/headers/HX-Target).
///
/// A flag is true when [`lookup`](Self::lookup) is given `Some` for that header. htmx 4.0.0 sends
/// `true`. The text of `HX-Source` and `HX-Target` is `tagName#id`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HxRequestInfo<'a> {
    /// [`HX-Boosted`](https://four.htmx.org/reference/headers/HX-Boosted) is present.
    pub boosted: bool,
    /// [`HX-Current-URL`](https://four.htmx.org/reference/headers/HX-Current-URL).
    pub current_url: Option<&'a str>,
    /// [`HX-History-Restore-Request`](https://four.htmx.org/reference/headers/HX-History-Restore-Request) is present.
    pub history_restore_request: bool,
    /// [`HX-Source`](https://four.htmx.org/reference/headers/HX-Source).
    pub source: Option<&'a str>,
    /// [`HX-Target`](https://four.htmx.org/reference/headers/HX-Target).
    pub target: Option<&'a str>,
}

impl<'a> HxRequestInfo<'a> {
    /// Reads the five headers through `get`. `get` receives the lowercase header name.
    #[must_use]
    pub fn lookup(mut get: impl FnMut(&str) -> Option<&'a str>) -> Self {
        Self {
            boosted: get(HX_BOOSTED).is_some(),
            current_url: get(HX_CURRENT_URL),
            history_restore_request: get(HX_HISTORY_RESTORE_REQUEST).is_some(),
            source: get(HX_SOURCE),
            target: get(HX_TARGET),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request_headers;

    #[test]
    fn lookup_reads_presence_and_text() {
        let info = HxRequestInfo::lookup(|name| match name {
            request_headers::HX_BOOSTED => Some("true"),
            request_headers::HX_CURRENT_URL => Some("https://example.test/orders"),
            request_headers::HX_SOURCE => Some("button#save"),
            request_headers::HX_TARGET => Some("div#results"),
            _ => None,
        });

        assert!(info.boosted);
        assert!(!info.history_restore_request);
        assert_eq!(info.current_url, Some("https://example.test/orders"));
        assert_eq!(info.source, Some("button#save"));
        assert_eq!(info.target, Some("div#results"));
    }
}
