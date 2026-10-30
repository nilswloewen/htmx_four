//! Core [response headers](https://four.htmx.org/reference/headers) a server sends back to htmx.

header_names! {
    /// [`HX-Location`](https://four.htmx.org/reference/headers/HX-Location): redirect without a full page load.
    HX_LOCATION = "hx-location";
    /// The htmx 2 `HX-Push` response header.
    ///
    /// Requires the `htmx-2-compat` feature. htmx 4 reads [`HX-Push-Url`](https://four.htmx.org/reference/headers/HX-Push-Url).
    #[cfg(feature = "htmx-2-compat")]
    HX_PUSH = "hx-push";
    /// [`HX-Push-Url`](https://four.htmx.org/reference/headers/HX-Push-Url): push a URL onto the browser history stack.
    HX_PUSH_URL = "hx-push-url";
    /// [`HX-Redirect`](https://four.htmx.org/reference/headers/HX-Redirect): redirect by setting `location.href`.
    HX_REDIRECT = "hx-redirect";
    /// [`HX-Refresh`](https://four.htmx.org/reference/headers/HX-Refresh): reload the page with `location.reload()`.
    HX_REFRESH = "hx-refresh";
    /// [`HX-Replace-Url`](https://four.htmx.org/reference/headers/HX-Replace-Url): replace the current history entry.
    HX_REPLACE_URL = "hx-replace-url";
    /// [`HX-Reselect`](https://four.htmx.org/reference/headers/HX-Reselect): override which content is swapped in.
    HX_RESELECT = "hx-reselect";
    /// [`HX-Reswap`](https://four.htmx.org/reference/headers/HX-Reswap): override the swap style.
    HX_RESWAP = "hx-reswap";
    /// [`HX-Retarget`](https://four.htmx.org/reference/headers/HX-Retarget): override the swap target.
    HX_RETARGET = "hx-retarget";
    /// [`HX-Trigger`](https://four.htmx.org/reference/headers/HX-Trigger): fire client-side events after the swap.
    HX_TRIGGER = "hx-trigger";
    /// The htmx 2 `HX-Trigger-After-Settle` response header.
    ///
    /// Requires the `htmx-2-compat` feature. htmx 4 uses [`HX-Trigger`](https://four.htmx.org/reference/headers/HX-Trigger) after the swap.
    /// See [response headers](https://four.htmx.org/docs/whats-new-in-htmx-4#response-headers).
    #[cfg(feature = "htmx-2-compat")]
    HX_TRIGGER_AFTER_SETTLE = "hx-trigger-after-settle";
    /// The htmx 2 `HX-Trigger-After-Swap` response header.
    ///
    /// Requires the `htmx-2-compat` feature. htmx 4 uses [`HX-Trigger`](https://four.htmx.org/reference/headers/HX-Trigger) after the swap.
    /// See [response headers](https://four.htmx.org/docs/whats-new-in-htmx-4#response-headers).
    #[cfg(feature = "htmx-2-compat")]
    HX_TRIGGER_AFTER_SWAP = "hx-trigger-after-swap";
}
