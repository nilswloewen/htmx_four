//! Headers on one part of a `multipart/mixed` response. Every core response header also works on
//! a part. See [`hx-multipart` headers](https://four.htmx.org/extensions/hx-multipart#headers).

use http::HeaderName;

/// [`HX-Part-ID`](https://four.htmx.org/extensions/hx-multipart#hx-part-id): identifies this part for a later reconnect.
pub const HX_PART_ID: HeaderName = HeaderName::from_static("hx-part-id");
/// [`HX-Select`](https://four.htmx.org/extensions/hx-multipart#hx--headers): selects the content this part swaps.
pub const HX_SELECT: HeaderName = HeaderName::from_static("hx-select");
/// [`HX-Swap`](https://four.htmx.org/extensions/hx-multipart#hx--headers): sets this part's swap style.
///
/// [`HX-Reswap`](https://four.htmx.org/reference/headers/HX-Reswap) on the same part overrides it.
pub const HX_SWAP: HeaderName = HeaderName::from_static("hx-swap");
/// [`HX-Target`](https://four.htmx.org/reference/headers/HX-Target) on one part.
///
/// This is [`crate::request_headers::HX_TARGET`]. On a part it names that part's swap target.
/// See [`HX-*` headers](https://four.htmx.org/extensions/hx-multipart#hx--headers).
#[doc(no_inline)]
pub use crate::request_headers::HX_TARGET;
