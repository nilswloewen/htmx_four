//! The [`HX-Download`](https://four.htmx.org/extensions/hx-download#hx-download-header) response header.

use http::HeaderName;

/// [`HX-Download`](https://four.htmx.org/extensions/hx-download#hx-download-header): a URL the extension fetches as the file.
pub const HX_DOWNLOAD: HeaderName = HeaderName::from_static("hx-download");
