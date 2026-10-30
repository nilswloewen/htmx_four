//! Core [request headers](https://four.htmx.org/reference/headers) htmx sends.

header_names! {
    /// [`HX-Boosted`](https://four.htmx.org/reference/headers/HX-Boosted): this request came from a boosted element.
    HX_BOOSTED = "hx-boosted";
    /// [`HX-Current-URL`](https://four.htmx.org/reference/headers/HX-Current-URL): the browser URL when the request started.
    HX_CURRENT_URL = "hx-current-url";
    /// [`HX-History-Restore-Request`](https://four.htmx.org/reference/headers/HX-History-Restore-Request): this request restores a history entry.
    HX_HISTORY_RESTORE_REQUEST = "hx-history-restore-request";
    /// [`HX-Prompt`](https://four.htmx.org/extensions/hx-prompt): the answer from the prompt.
    ///
    /// htmx 4 sends this only when the `hx-prompt` extension is loaded.
    HX_PROMPT = "hx-prompt";
    /// [`HX-Request`](https://four.htmx.org/reference/headers/HX-Request): this request was made by htmx. The value is `true`.
    HX_REQUEST = "hx-request";
    /// [`HX-Request-Type`](https://four.htmx.org/reference/headers/HX-Request-Type): `partial` or `full`.
    HX_REQUEST_TYPE = "hx-request-type";
    /// [`HX-Source`](https://four.htmx.org/reference/headers/HX-Source): the element that triggered the request, as `tagName#id`.
    HX_SOURCE = "hx-source";
    /// [`HX-Target`](https://four.htmx.org/reference/headers/HX-Target): the element that will receive the response, as `tagName#id`.
    HX_TARGET = "hx-target";
    /// The htmx 2 `HX-Trigger` request header.
    ///
    /// Requires the `htmx-2-compat` feature. htmx 4 sends [`HX-Source`](https://four.htmx.org/reference/headers/HX-Source) instead.
    /// See [request headers](https://four.htmx.org/docs/whats-new-in-htmx-4#request-headers).
    #[cfg(feature = "htmx-2-compat")]
    HX_TRIGGER = "hx-trigger";
    /// The htmx 2 `HX-Trigger-Name` request header.
    ///
    /// Requires the `htmx-2-compat` feature. htmx 4 removed it. The triggering element is [`HX-Source`](https://four.htmx.org/reference/headers/HX-Source).
    /// See [request headers](https://four.htmx.org/docs/whats-new-in-htmx-4#request-headers).
    #[cfg(feature = "htmx-2-compat")]
    HX_TRIGGER_NAME = "hx-trigger-name";
}
