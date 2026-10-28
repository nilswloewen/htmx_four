//! [`hx-multipart`](https://four.htmx.org/extensions/hx-multipart) streams HTML with `multipart/mixed`.

mod multipart_config;
pub mod part_headers;
pub mod request_headers;

pub use multipart_config::MultipartConfig;
