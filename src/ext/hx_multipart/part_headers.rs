//! Headers on one part of a `multipart/mixed` response. Every core response header also works on
//! a part. See [`hx-multipart` headers](https://four.htmx.org/extensions/hx-multipart#headers).

header_names! {
    /// [`HX-Part-ID`](https://four.htmx.org/extensions/hx-multipart#hx-part-id): identifies this part for a later reconnect.
    HX_PART_ID = "hx-part-id";
    /// [`HX-Select`](https://four.htmx.org/extensions/hx-multipart#hx--headers): selects the content this part swaps.
    HX_SELECT = "hx-select";
    /// [`HX-Swap`](https://four.htmx.org/extensions/hx-multipart#hx--headers): sets this part's swap style.
    ///
    /// [`HX-Reswap`](https://four.htmx.org/reference/headers/HX-Reswap) on the same part overrides it.
    HX_SWAP = "hx-swap";
    @reexports {
        /// [`HX-Target`](https://four.htmx.org/reference/headers/HX-Target) on one part.
        ///
        /// This is [`crate::request_headers::HX_TARGET`]. On a part it names that part's swap target.
        /// See [`HX-*` headers](https://four.htmx.org/extensions/hx-multipart#hx--headers).
        #[doc(no_inline)]
        HX_TARGET from crate::request_headers;
    }
}
