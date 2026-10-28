//! Core [request headers](https://four.htmx.org/reference/headers) htmx sends.

use http::HeaderName;

/// [`HX-Boosted`](https://four.htmx.org/reference/headers/HX-Boosted): this request came from a boosted element.
pub const HX_BOOSTED: HeaderName = HeaderName::from_static("hx-boosted");
/// [`HX-Current-URL`](https://four.htmx.org/reference/headers/HX-Current-URL): the browser URL when the request started.
pub const HX_CURRENT_URL: HeaderName = HeaderName::from_static("hx-current-url");
/// [`HX-History-Restore-Request`](https://four.htmx.org/reference/headers/HX-History-Restore-Request): this request restores a history entry.
pub const HX_HISTORY_RESTORE_REQUEST: HeaderName =
    HeaderName::from_static("hx-history-restore-request");
/// [`HX-Prompt`](https://four.htmx.org/extensions/hx-prompt): the answer from the prompt.
///
/// htmx 4 sends this only when the `hx-prompt` extension is loaded.
pub const HX_PROMPT: HeaderName = HeaderName::from_static("hx-prompt");
/// [`HX-Request`](https://four.htmx.org/reference/headers/HX-Request): this request was made by htmx. The value is `true`.
pub const HX_REQUEST: HeaderName = HeaderName::from_static("hx-request");
/// [`HX-Request-Type`](https://four.htmx.org/reference/headers/HX-Request-Type): `partial` or `full`.
pub const HX_REQUEST_TYPE: HeaderName = HeaderName::from_static("hx-request-type");
/// [`HX-Source`](https://four.htmx.org/reference/headers/HX-Source): the element that triggered the request, as `tagName#id`.
pub const HX_SOURCE: HeaderName = HeaderName::from_static("hx-source");
/// [`HX-Target`](https://four.htmx.org/reference/headers/HX-Target): the element that will receive the response, as `tagName#id`.
pub const HX_TARGET: HeaderName = HeaderName::from_static("hx-target");
/// The htmx 2 `HX-Trigger` request header.
///
/// htmx 4 sends [`HX-Source`](https://four.htmx.org/reference/headers/HX-Source) instead.
/// See [request headers](https://four.htmx.org/docs/whats-new-in-htmx-4#request-headers).
pub const HX_TRIGGER: HeaderName = HeaderName::from_static("hx-trigger");
/// The htmx 2 `HX-Trigger-Name` request header.
///
/// htmx 4 removed it. The triggering element is [`HX-Source`](https://four.htmx.org/reference/headers/HX-Source).
/// See [request headers](https://four.htmx.org/docs/whats-new-in-htmx-4#request-headers).
pub const HX_TRIGGER_NAME: HeaderName = HeaderName::from_static("hx-trigger-name");
