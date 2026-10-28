use bon::Builder;
use serde::Serialize;

/// The `live` section of the htmx config, read by the `hx-live` extension.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct LiveConfig {
    /// An empty prefix turns the short form off, so only `hx-live:<attr>` binds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_debounce: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_dollar: Option<bool>,
}
