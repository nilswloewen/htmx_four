//! The [`HX-Preloaded`](https://four.htmx.org/extensions/hx-preload#usage) request header.

header_names! {
    /// [`HX-Preloaded`](https://four.htmx.org/extensions/hx-preload#usage): the docs say preload requests send `true`.
    ///
    /// The 4.0.1 extension does not send it (bigskysoftware/htmx#4110). It does set
    /// [`HX-Request-Type`](https://four.htmx.org/reference/headers/HX-Request-Type) to `full` when the
    /// target is the document body or the request selects a fragment, and to `partial` otherwise.
    HX_PRELOADED = "hx-preloaded";
}
