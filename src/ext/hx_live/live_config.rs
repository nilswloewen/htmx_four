use bon::Builder;
use serde::Serialize;

/// The `live` section of [`htmx.config`](https://four.htmx.org/reference/config/htmx-config), read by [`hx-live`](https://four.htmx.org/extensions/hx-live#configuration).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct LiveConfig {
    /// [`bindPrefix`](https://four.htmx.org/extensions/hx-live#configlivebindprefix): the short-form prefix for bindings.
    ///
    /// An empty prefix turns the short form off, so only `hx-live:<attr>` binds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_prefix: Option<String>,

    /// [`inputDebounce`](https://four.htmx.org/extensions/hx-live#configliveinputdebounce): how long to wait after an `input` event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_debounce: Option<String>,

    /// [`useDollar`](https://four.htmx.org/extensions/hx-live#configliveusedollar): allow `$()` as an alias of `q()`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_dollar: Option<bool>,
}
