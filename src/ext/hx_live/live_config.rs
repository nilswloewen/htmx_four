use serde::Serialize;
use typed_builder::TypedBuilder;

/// The `live` section of the htmx config, read by the `hx-live` extension.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LiveConfig {
    /// An empty prefix turns the short form off, so only `hx-live:<attr>` binds.
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_prefix: Option<String>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_debounce: Option<String>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_dollar: Option<bool>,
}
