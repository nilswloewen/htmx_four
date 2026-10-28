use crate::extension::serialize_extensions;
use crate::{
    CompatConfig, Extension, History, HistoryCacheConfig, LiveConfig, MultipartConfig,
    PreloadConfig, RequestMode, SseConfig, StatusPattern, SwapStyle, WsConfig,
};
use bon::Builder;
use serde::Serialize;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct HtmxConfig {
    #[serde(
        rename = "allowEmptySwapAfterOOB",
        skip_serializing_if = "Option::is_none"
    )]
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

    #[serde(
        serialize_with = "serialize_extensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub extensions: Option<Vec<Extension>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<History>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_cache: Option<HistoryCacheConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_inheritance: Option<bool>,

    #[serde(
        rename = "includeIndicatorCSS",
        skip_serializing_if = "Option::is_none"
    )]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SseReleaseOn;
    use http::StatusCode;

    #[test]
    fn default_config_serializes_to_empty_object() {
        let json = HtmxConfig::default().to_json().unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn full_config_serializes_every_extension() {
        let config = HtmxConfig::builder()
            .allow_empty_swap_after_oob(true)
            .boost_browser_indicator(true)
            .compat(
                CompatConfig::builder()
                    .do_not_trigger_old_events(true)
                    .suppress_inheritance_logs(true)
                    .swap_error_response_codes(false)
                    .use_explicit_inheritance(true)
                    .build(),
            )
            .default_focus_scroll(false)
            .default_settle_delay(20)
            .default_swap(SwapStyle::OuterMorph)
            .default_timeout(5000)
            .extensions(vec![
                Extension::Htmx2Compat,
                Extension::HxAlpineCompat,
                Extension::HxBrowserIndicator,
                Extension::HxCsp,
                Extension::HxDownload,
                Extension::HxHead,
                Extension::HxHistoryCache,
                Extension::HxLive,
                Extension::HxMultipart,
                Extension::HxPending,
                Extension::HxPreload,
                Extension::HxPrompt,
                Extension::HxPtag,
                Extension::HxSse,
                Extension::HxTargets,
                Extension::HxUpsert,
                Extension::HxWs,
            ])
            .history(History::Reload)
            .history_cache(
                HistoryCacheConfig::builder()
                    .disable(false)
                    .refresh_on_miss(true)
                    .size(50)
                    .swap_style(SwapStyle::Upsert)
                    .build(),
            )
            .implicit_inheritance(false)
            .include_indicator_css(false)
            .indicator_class("htmx-indicator")
            .inline_script_nonce("nonce-1")
            .live(
                LiveConfig::builder()
                    .bind_prefix("data-")
                    .input_debounce("200ms")
                    .use_dollar(true)
                    .build(),
            )
            .log_all(true)
            .meta_character("hx-")
            .mode(RequestMode::NoCors)
            .morph_ignore(vec!["data-skip".to_owned()])
            .morph_scan_limit(100)
            .morph_skip("ignore")
            .morph_skip_children("skip-kids")
            .multipart(
                MultipartConfig::builder()
                    .pause_on_background(true)
                    .reconnect(true)
                    .reconnect_delay("100ms")
                    .reconnect_jitter(0.5)
                    .reconnect_max_attempts(8)
                    .reconnect_max_delay("4s")
                    .build(),
            )
            .no_swap(vec![
                StatusPattern::ClientError,
                StatusPattern::Exact(StatusCode::NOT_FOUND),
            ])
            .prefix("hx-")
            .preload(
                PreloadConfig::builder()
                    .auto_boost(true)
                    .boost_event("mouseover")
                    .boost_timeout("1s")
                    .build(),
            )
            .request_class("htmx-request")
            .safe_eval(false)
            .sse(
                SseConfig::builder()
                    .pause_on_background(true)
                    .reconnect(true)
                    .reconnect_delay("500ms")
                    .reconnect_jitter(0.5)
                    .reconnect_max_attempts(4)
                    .reconnect_max_delay("8s")
                    .release_on(SseReleaseOn::End)
                    .build(),
            )
            .transitions(true)
            .ws(WsConfig::builder()
                .max_outgoing_messages_queue_size(16)
                .pause_on_background(true)
                .protocols(vec!["chat".to_owned()])
                .reconnect(true)
                .reconnect_codes(vec![1001, 1012])
                .reconnect_delay("1s")
                .reconnect_jitter(0.5)
                .reconnect_max_attempts(10)
                .reconnect_max_delay("30s")
                .build())
            .build();

        let value: serde_json::Value = serde_json::from_str(&config.to_json().unwrap()).unwrap();
        assert_eq!(
            value,
            serde_json::json!({
                "allowEmptySwapAfterOOB": true,
                "boostBrowserIndicator": true,
                "compat": {
                    "doNotTriggerOldEvents": true,
                    "suppressInheritanceLogs": true,
                    "swapErrorResponseCodes": false,
                    "useExplicitInheritace": true
                },
                "defaultFocusScroll": false,
                "defaultSettleDelay": 20,
                "defaultSwap": "outerMorph",
                "defaultTimeout": 5000,
                "extensions": "compat, alpine-compat, browser-indicator, hx-csp, download, hx-head, history-cache, hx-live, hx-multipart, hx-pending, preload, hx-prompt, ptag, sse, hx-targets, upsert, ws",
                "history": "reload",
                "historyCache": {
                    "disable": false,
                    "refreshOnMiss": true,
                    "size": 50,
                    "swapStyle": "upsert"
                },
                "implicitInheritance": false,
                "includeIndicatorCSS": false,
                "indicatorClass": "htmx-indicator",
                "inlineScriptNonce": "nonce-1",
                "live": {
                    "bindPrefix": "data-",
                    "inputDebounce": "200ms",
                    "useDollar": true
                },
                "logAll": true,
                "metaCharacter": "hx-",
                "mode": "no-cors",
                "morphIgnore": ["data-skip"],
                "morphScanLimit": 100,
                "morphSkip": "ignore",
                "morphSkipChildren": "skip-kids",
                "multipart": {
                    "pauseOnBackground": true,
                    "reconnect": true,
                    "reconnectDelay": "100ms",
                    "reconnectJitter": 0.5,
                    "reconnectMaxAttempts": 8,
                    "reconnectMaxDelay": "4s"
                },
                "noSwap": ["4xx", 404],
                "prefix": "hx-",
                "preload": {
                    "autoBoost": true,
                    "boostEvent": "mouseover",
                    "boostTimeout": "1s"
                },
                "requestClass": "htmx-request",
                "safeEval": false,
                "sse": {
                    "pauseOnBackground": true,
                    "reconnect": true,
                    "reconnectDelay": "500ms",
                    "reconnectJitter": 0.5,
                    "reconnectMaxAttempts": 4,
                    "reconnectMaxDelay": "8s",
                    "releaseOn": "end"
                },
                "transitions": true,
                "ws": {
                    "maxOutgoingMessagesQueueSize": 16,
                    "pauseOnBackground": true,
                    "protocols": ["chat"],
                    "reconnect": true,
                    "reconnectCodes": [1001, 1012],
                    "reconnectDelay": "1s",
                    "reconnectJitter": 0.5,
                    "reconnectMaxAttempts": 10,
                    "reconnectMaxDelay": "30s"
                }
            })
        );
    }
}
