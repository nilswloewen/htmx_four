use crate::extension::serialize_extensions;
use crate::{
    CompatConfig, Extension, HistoryCacheConfig, History, LiveConfig, MultipartConfig, PreloadConfig, RequestMode, SseConfig,
    StatusPattern, SwapStyle, WsConfig,
};
use bon::Builder;
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct HtmxConfig {
    #[serde(rename = "allowEmptySwapAfterOOB", skip_serializing_if = "Option::is_none")]
    pub allow_empty_swap_after_oob: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_browser_indicator: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compat: Option<CompatConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_focus_scroll: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settle_delay: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_swap: Option<SwapStyle>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_timeout: Option<u32>,

    #[serde(serialize_with = "serialize_extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<History>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_cache: Option<HistoryCacheConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_inheritance: Option<bool>,

    #[serde(rename = "includeIndicatorCSS", skip_serializing_if = "Option::is_none")]
    pub include_indicator_css: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicator_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_script_nonce: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub live: Option<LiveConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_all: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_character: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<RequestMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_ignore: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_scan_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_skip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_skip_children: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart: Option<MultipartConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_swap: Option<Vec<StatusPattern>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload: Option<PreloadConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_class: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_eval: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse: Option<SseConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws: Option<WsConfig>,
}

impl HtmxConfig {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}
