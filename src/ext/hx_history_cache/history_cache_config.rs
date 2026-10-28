use crate::SwapStyle;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// The `historyCache` section of the htmx config, read by the `hx-history-cache` extension.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct HistoryCacheConfig {
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_on_miss: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_style: Option<SwapStyle>,
}
