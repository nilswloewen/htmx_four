//! The [`HX-Preloaded`](https://four.htmx.org/extensions/hx-preload#usage) request header.

header_names! {
    /// [`HX-Preloaded`](https://four.htmx.org/extensions/hx-preload#usage): the docs say preload requests send `true`.
    ///
    /// The 4.0.0 extension does not send it (bigskysoftware/htmx#4110).
    HX_PRELOADED = "hx-preloaded";
}
