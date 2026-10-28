//! Extensions that ship with [htmx 4.0.0](https://four.htmx.org/extensions).
//!
//! Module names follow the script files in `src/ext`. Config sections are also re-exported from the crate root.

/// [`htmx-2-compat`](https://four.htmx.org/extensions/htmx-2-compat), registered as `compat`.
pub mod htmx_2_compat;
/// [`hx-download`](https://four.htmx.org/extensions/hx-download), registered as `download`.
pub mod hx_download;
/// [`hx-history-cache`](https://four.htmx.org/extensions/hx-history-cache), registered as `history-cache`.
pub mod hx_history_cache;
/// [`hx-live`](https://four.htmx.org/extensions/hx-live), registered as `hx-live`.
pub mod hx_live;
/// [`hx-multipart`](https://four.htmx.org/extensions/hx-multipart), registered as `hx-multipart`.
pub mod hx_multipart;
/// [`hx-preload`](https://four.htmx.org/extensions/hx-preload), registered as `preload`.
pub mod hx_preload;
/// [`hx-prompt`](https://four.htmx.org/extensions/hx-prompt), registered as `hx-prompt`.
pub mod hx_prompt;
/// [`hx-ptag`](https://four.htmx.org/extensions/hx-ptag), registered as `ptag`.
pub mod hx_ptag;
/// [`hx-sse`](https://four.htmx.org/extensions/hx-sse), registered as `sse`.
pub mod hx_sse;
/// [`hx-ws`](https://four.htmx.org/extensions/hx-ws), registered as `ws`.
pub mod hx_ws;
