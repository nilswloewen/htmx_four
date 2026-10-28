use bon::Builder;
use serde::Serialize;

/// The `compat` section of the htmx config, read by the `htmx-2-compat` extension.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CompatConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_trigger_old_events: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_inheritance_logs: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_error_response_codes: Option<bool>,

    /// The 4.0.0 extension reads the misspelled key `useExplicitInheritace`.
    #[serde(
        rename = "useExplicitInheritace",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_explicit_inheritance: Option<bool>,
}
