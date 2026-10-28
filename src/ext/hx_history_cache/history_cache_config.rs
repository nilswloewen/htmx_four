use crate::SwapStyle;
use bon::Builder;
use serde::Serialize;

/// The `historyCache` section of the htmx config, read by the `hx-history-cache` extension.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct HistoryCacheConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_on_miss: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_style: Option<SwapStyle>,
}
