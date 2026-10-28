use bon::Builder;
use serde::Serialize;

/// The `compat` section of [`htmx.config`](https://four.htmx.org/reference/config/htmx-config), read by [`htmx-2-compat`](https://four.htmx.org/extensions/htmx-2-compat).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct CompatConfig {
    /// `doNotTriggerOldEvents`: skip the htmx 2 event names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub do_not_trigger_old_events: Option<bool>,

    /// `suppressInheritanceLogs`: skip the implicit-inheritance console message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_inheritance_logs: Option<bool>,

    /// `swapErrorResponseCodes`: keep swapping `4xx` and `5xx` responses.
    ///
    /// When this is unset, the extension adds those classes to [`noSwap`](https://four.htmx.org/reference/config/htmx-config-noSwap).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_error_response_codes: Option<bool>,

    /// `useExplicitInheritace`: keep htmx 4 inheritance.
    ///
    /// The 4.0.0 extension reads that misspelled key. When this is unset, the extension turns
    /// [`implicitInheritance`](https://four.htmx.org/reference/config/htmx-config-implicitInheritance) on.
    #[serde(
        rename = "useExplicitInheritace",
        skip_serializing_if = "Option::is_none"
    )]
    pub use_explicit_inheritance: Option<bool>,
}
