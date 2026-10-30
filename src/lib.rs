#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod header_names;

pub mod ext;
mod extension;
mod history;
mod htmx_config;
pub mod request_headers;
mod request_mode;
pub mod response_headers;
mod status_pattern;
mod swap_style;

pub use ext::htmx_2_compat::CompatConfig;
pub use ext::hx_history_cache::HistoryCacheConfig;
pub use ext::hx_live::LiveConfig;
pub use ext::hx_multipart::MultipartConfig;
pub use ext::hx_preload::PreloadConfig;
pub use ext::hx_sse::{SseConfig, SseReleaseOn};
pub use ext::hx_ws::WsConfig;
pub use extension::Extension;
pub use history::History;
pub use htmx_config::HtmxConfig;
pub use request_mode::RequestMode;
pub use status_pattern::StatusPattern;
pub use swap_style::SwapStyle;
