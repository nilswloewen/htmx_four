#![doc = include_str!("../README.md")]

#[macro_use]
mod header_names;

pub mod ext;
mod extension;
mod history;
mod htmx_config;
mod hx_event;
mod hx_location;
pub mod request_headers;
mod request_info;
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
pub use hx_event::{HxEvent, HxEventError, HxTrigger, HxTriggerParseError};
pub use hx_location::{HxLocation, HxLocationError, HxUrlHistory};
pub use request_info::HxRequestInfo;
pub use request_mode::RequestMode;
pub use status_pattern::StatusPattern;
pub use swap_style::SwapStyle;
