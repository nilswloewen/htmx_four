//! Headers on one part of a `multipart/mixed` response. Every core response header also works on
//! a part.

use http::HeaderName;

pub const HX_PART_ID: HeaderName = HeaderName::from_static("hx-part-id");
pub const HX_SELECT: HeaderName = HeaderName::from_static("hx-select");
pub const HX_SWAP: HeaderName = HeaderName::from_static("hx-swap");
pub use crate::request_headers::HX_TARGET;
