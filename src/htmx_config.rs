use crate::extension::serialize_extensions;
use crate::{
    CompatConfig, Extension, HistoryCacheConfig, History, LiveConfig, MultipartConfig, PreloadConfig, RequestMode, SseConfig,
    StatusPattern, SwapStyle, WsConfig,
};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, Default, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct HtmxConfig {
    #[builder(default, setter(strip_option))]
    #[serde(rename = "allowEmptySwapAfterOOB", skip_serializing_if = "Option::is_none")]
    pub allow_empty_swap_after_oob: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_browser_indicator: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compat: Option<CompatConfig>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_focus_scroll: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settle_delay: Option<u32>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_swap: Option<SwapStyle>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_timeout: Option<u32>,

    #[builder(default, setter(strip_option))]
    #[serde(serialize_with = "serialize_extensions", skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<Extension>>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<History>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_cache: Option<HistoryCacheConfig>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_inheritance: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[serde(rename = "includeIndicatorCSS", skip_serializing_if = "Option::is_none")]
    pub include_indicator_css: Option<bool>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicator_class: Option<String>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_script_nonce: Option<String>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live: Option<LiveConfig>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_all: Option<bool>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_character: Option<String>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<RequestMode>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_ignore: Option<Vec<String>>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_scan_limit: Option<u32>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_skip: Option<String>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_skip_children: Option<String>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart: Option<MultipartConfig>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_swap: Option<Vec<StatusPattern>>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload: Option<PreloadConfig>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_class: Option<String>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_eval: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse: Option<SseConfig>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws: Option<WsConfig>,
}

impl HtmxConfig {
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}
