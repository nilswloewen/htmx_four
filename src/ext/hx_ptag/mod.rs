use http::HeaderName;

pub const HX_PTAG: HeaderName = HeaderName::from_static("hx-ptag");

pub mod request_headers;
pub mod response_headers;
