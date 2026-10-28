use crate::SwapStyle;
use bon::Builder;
use serde::Serialize;

/// The `historyCache` section of [`htmx.config`](https://four.htmx.org/reference/config/htmx-config), read by [`hx-history-cache`](https://four.htmx.org/extensions/hx-history-cache#configuration).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct HistoryCacheConfig {
    /// `disable`: turn the cache off without unloading the extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,

    /// `refreshOnMiss`: reload the page when the history entry is not cached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_on_miss: Option<bool>,

    /// `size`: how many pages to keep. `0` disables the cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,

    /// `swapStyle`: how a cached page is restored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_style: Option<SwapStyle>,
}
