//! Core [response headers](https://four.htmx.org/reference/headers) a server sends back to htmx.

use http::HeaderName;

/// [`HX-Location`](https://four.htmx.org/reference/headers/HX-Location): redirect without a full page load.
pub const HX_LOCATION: HeaderName = HeaderName::from_static("hx-location");
/// [`HX-Push`](https://four.htmx.org/reference/headers/HX-Push): the old name for [`HX-Push-Url`](https://four.htmx.org/reference/headers/HX-Push-Url).
pub const HX_PUSH: HeaderName = HeaderName::from_static("hx-push");
/// [`HX-Push-Url`](https://four.htmx.org/reference/headers/HX-Push-Url): push a URL onto the browser history stack.
pub const HX_PUSH_URL: HeaderName = HeaderName::from_static("hx-push-url");
/// [`HX-Redirect`](https://four.htmx.org/reference/headers/HX-Redirect): redirect by setting `location.href`.
pub const HX_REDIRECT: HeaderName = HeaderName::from_static("hx-redirect");
/// [`HX-Refresh`](https://four.htmx.org/reference/headers/HX-Refresh): reload the page with `location.reload()`.
pub const HX_REFRESH: HeaderName = HeaderName::from_static("hx-refresh");
/// [`HX-Replace-Url`](https://four.htmx.org/reference/headers/HX-Replace-Url): replace the current history entry.
pub const HX_REPLACE_URL: HeaderName = HeaderName::from_static("hx-replace-url");
/// [`HX-Reselect`](https://four.htmx.org/reference/headers/HX-Reselect): override which content is swapped in.
pub const HX_RESELECT: HeaderName = HeaderName::from_static("hx-reselect");
/// [`HX-Reswap`](https://four.htmx.org/reference/headers/HX-Reswap): override the swap style.
pub const HX_RESWAP: HeaderName = HeaderName::from_static("hx-reswap");
/// [`HX-Retarget`](https://four.htmx.org/reference/headers/HX-Retarget): override the swap target.
pub const HX_RETARGET: HeaderName = HeaderName::from_static("hx-retarget");
/// [`HX-Trigger`](https://four.htmx.org/reference/headers/HX-Trigger): fire client-side events after the swap.
pub const HX_TRIGGER: HeaderName = HeaderName::from_static("hx-trigger");
/// The htmx 2 `HX-Trigger-After-Settle` response header.
///
/// htmx 4 uses [`HX-Trigger`](https://four.htmx.org/reference/headers/HX-Trigger) after the swap.
/// See [response headers](https://four.htmx.org/docs/whats-new-in-htmx-4#response-headers).
pub const HX_TRIGGER_AFTER_SETTLE: HeaderName = HeaderName::from_static("hx-trigger-after-settle");
/// The htmx 2 `HX-Trigger-After-Swap` response header.
///
/// htmx 4 uses [`HX-Trigger`](https://four.htmx.org/reference/headers/HX-Trigger) after the swap.
/// See [response headers](https://four.htmx.org/docs/whats-new-in-htmx-4#response-headers).
pub const HX_TRIGGER_AFTER_SWAP: HeaderName = HeaderName::from_static("hx-trigger-after-swap");
