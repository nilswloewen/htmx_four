use crate::extension::serialize_extensions;
use crate::{
    CompatConfig, Extension, History, HistoryCacheConfig, LiveConfig, MultipartConfig,
    PreloadConfig, RequestMode, SseConfig, StatusPattern, SwapStyle, WsConfig,
};
use bon::Builder;
use serde::Serialize;

/// The [`htmx.config`](https://four.htmx.org/reference/config/htmx-config) object.
///
/// [`to_json`](Self::to_json) writes the JSON htmx reads from `<meta name="htmx-config">`.
/// Fields left unset are omitted.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct HtmxConfig {
    /// [`allowEmptySwapAfterOOB`](https://four.htmx.org/reference/config/htmx-config-allowEmptySwapAfterOOB): allow an empty main swap after out-of-band extraction.
    #[serde(
        rename = "allowEmptySwapAfterOOB",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_empty_swap_after_oob: Option<bool>,

    /// [`boostBrowserIndicator`](https://four.htmx.org/extensions/hx-browser-indicator#boosted-elements): show the tab spinner for boosted requests.
    ///
    /// Read by `hx-browser-indicator`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_browser_indicator: Option<bool>,

    /// The [`compat`](https://four.htmx.org/extensions/htmx-2-compat) section, read by `htmx-2-compat`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compat: Option<CompatConfig>,

    /// [`defaultFocusScroll`](https://four.htmx.org/reference/config/htmx-config-defaultFocusScroll): scroll a focused element into view after a swap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_focus_scroll: Option<bool>,

    /// [`defaultSettleDelay`](https://four.htmx.org/reference/config/htmx-config-defaultSettleDelay): milliseconds before a swap settles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settle_delay: Option<u32>,

    /// [`defaultSwap`](https://four.htmx.org/reference/config/htmx-config-defaultSwap): the swap style used when `hx-swap` is absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_swap: Option<SwapStyle>,

    /// [`defaultTimeout`](https://four.htmx.org/reference/config/htmx-config-defaultTimeout): request timeout in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_timeout: Option<u32>,

    /// [`extensions`](https://four.htmx.org/reference/config/htmx-config-extensions): extensions to load, serialized as a comma-separated list.
    #[serde(
        serialize_with = "serialize_extensions",
        skip_serializing_if = "Option::is_none"
    )]
    pub extensions: Option<Vec<Extension>>,

    /// [`history`](https://four.htmx.org/reference/config/htmx-config-history): how htmx treats browser history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<History>,

    /// The [`historyCache`](https://four.htmx.org/extensions/hx-history-cache#configuration) section, read by `hx-history-cache`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_cache: Option<HistoryCacheConfig>,

    /// [`implicitInheritance`](https://four.htmx.org/reference/config/htmx-config-implicitInheritance): inherit attributes without an explicit modifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit_inheritance: Option<bool>,

    /// [`includeIndicatorCSS`](https://four.htmx.org/reference/config/htmx-config-includeIndicatorCSS): include the default indicator CSS.
    #[serde(
        rename = "includeIndicatorCSS",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_indicator_css: Option<bool>,

    /// [`indicatorClass`](https://four.htmx.org/reference/config/htmx-config-indicatorClass): the CSS class applied to indicators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicator_class: Option<String>,

    /// [`inlineScriptNonce`](https://four.htmx.org/reference/config/htmx-config-inlineScriptNonce): nonce added to inline scripts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_script_nonce: Option<String>,

    /// The [`live`](https://four.htmx.org/extensions/hx-live#configuration) section, read by `hx-live`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live: Option<LiveConfig>,

    /// [`logAll`](https://four.htmx.org/reference/config/htmx-config-logAll): log every htmx event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_all: Option<bool>,

    /// [`metaCharacter`](https://four.htmx.org/reference/config/htmx-config-metaCharacter): character used instead of `:` in attribute names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_character: Option<String>,

    /// [`mode`](https://four.htmx.org/reference/config/htmx-config-mode): the `mode` passed to `fetch()`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<RequestMode>,

    /// [`morphIgnore`](https://four.htmx.org/reference/config/htmx-config-morphIgnore): attributes ignored while morphing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_ignore: Option<Vec<String>>,

    /// [`morphScanLimit`](https://four.htmx.org/reference/config/htmx-config-morphScanLimit): how many siblings are scanned while matching elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_scan_limit: Option<u32>,

    /// [`morphSkip`](https://four.htmx.org/reference/config/htmx-config-morphSkip): selector for elements skipped while morphing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_skip: Option<String>,

    /// [`morphSkipChildren`](https://four.htmx.org/reference/config/htmx-config-morphSkipChildren): selector for elements whose children are not morphed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub morph_skip_children: Option<String>,

    /// The [`multipart`](https://four.htmx.org/extensions/hx-multipart#config) section, read by `hx-multipart`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart: Option<MultipartConfig>,

    /// [`noSwap`](https://four.htmx.org/reference/config/htmx-config-noSwap): statuses that skip the swap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_swap: Option<Vec<StatusPattern>>,

    /// [`prefix`](https://four.htmx.org/reference/config/htmx-config-prefix): a second attribute prefix, recognized alongside `hx-`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,

    /// The [`preload`](https://four.htmx.org/extensions/hx-preload#configuration) section, read by `hx-preload`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload: Option<PreloadConfig>,

    /// [`requestClass`](https://four.htmx.org/reference/config/htmx-config-requestClass): the CSS class applied during a request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_class: Option<String>,

    /// [`safeEval`](https://four.htmx.org/extensions/hx-csp#safe-eval): replace `new Function()` with nonce-based script injection.
    ///
    /// Read by `hx-csp`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_eval: Option<bool>,

    /// The [`sse`](https://four.htmx.org/extensions/hx-sse#config) section, read by `hx-sse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse: Option<SseConfig>,

    /// [`transitions`](https://four.htmx.org/reference/config/htmx-config-transitions): enable view transitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<bool>,

    /// The [`ws`](https://four.htmx.org/extensions/hx-ws#config) section, read by `hx-ws`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws: Option<WsConfig>,
}

impl HtmxConfig {
    /// Serializes this config as the JSON object htmx reads from `<meta name="htmx-config">`.
    ///
    /// See [`htmx.config`](https://four.htmx.org/reference/config/htmx-config).
    ///
    /// # Errors
    ///
    /// Returns an error when a floating-point field is `NaN` or infinite.
    /// JSON has no encoding for those values. Every other field of this struct serializes.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SseReleaseOn;

    #[test]
    fn default_config_serializes_to_empty_object() {
        let json = HtmxConfig::default().to_json().unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn full_config_serializes_every_extension() {
        let value: serde_json::Value =
            serde_json::from_str(&full_config().to_json().unwrap()).unwrap();
        assert_eq!(value, full_config_json());
    }

    fn full_config() -> HtmxConfig {
        HtmxConfig::builder()
            .allow_empty_swap_after_oob(true)
            .boost_browser_indicator(true)
            .compat(compat_config())
            .default_focus_scroll(false)
            .default_settle_delay(20)
            .default_swap(SwapStyle::OuterMorph)
            .default_timeout(5000)
            .extensions(official_extensions())
            .history(History::Reload)
            .history_cache(history_cache_config())
            .implicit_inheritance(false)
            .include_indicator_css(false)
            .indicator_class("htmx-indicator")
            .inline_script_nonce("nonce-1")
            .live(live_config())
            .log_all(true)
            .meta_character("hx-")
            .mode(RequestMode::NoCors)
            .morph_ignore(vec!["data-skip".to_owned()])
            .morph_scan_limit(100)
            .morph_skip("ignore")
            .morph_skip_children("skip-kids")
            .multipart(multipart_config())
            .no_swap(vec![StatusPattern::ClientError, StatusPattern::Exact(404)])
            .prefix("hx-")
            .preload(preload_config())
            .request_class("htmx-request")
            .safe_eval(false)
            .sse(sse_config())
            .transitions(true)
            .ws(ws_config())
            .build()
    }

    fn official_extensions() -> Vec<Extension> {
        vec![
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
        ]
    }

    fn compat_config() -> CompatConfig {
        CompatConfig::builder()
            .do_not_trigger_old_events(true)
            .suppress_inheritance_logs(true)
            .swap_error_response_codes(false)
            .use_explicit_inheritance(true)
            .build()
    }

    fn history_cache_config() -> HistoryCacheConfig {
        HistoryCacheConfig::builder()
            .disable(false)
            .refresh_on_miss(true)
            .size(50)
            .swap_style(SwapStyle::Upsert)
            .build()
    }

    fn live_config() -> LiveConfig {
        LiveConfig::builder()
            .bind_prefix("data-")
            .input_debounce("200ms")
            .use_dollar(true)
            .build()
    }

    fn multipart_config() -> MultipartConfig {
        MultipartConfig::builder()
            .pause_on_background(true)
            .reconnect(true)
            .reconnect_delay("100ms")
            .reconnect_jitter(0.5)
            .reconnect_max_attempts(8)
            .reconnect_max_delay("4s")
            .build()
    }

    fn preload_config() -> PreloadConfig {
        PreloadConfig::builder()
            .auto_boost(true)
            .boost_event("mouseover")
            .boost_timeout("1s")
            .build()
    }

    fn sse_config() -> SseConfig {
        SseConfig::builder()
            .pause_on_background(true)
            .reconnect(true)
            .reconnect_delay("500ms")
            .reconnect_jitter(0.5)
            .reconnect_max_attempts(4)
            .reconnect_max_delay("8s")
            .release_on(SseReleaseOn::End)
            .build()
    }

    fn ws_config() -> WsConfig {
        WsConfig::builder()
            .max_outgoing_messages_queue_size(16)
            .pause_on_background(true)
            .protocols(vec!["chat".to_owned()])
            .reconnect(true)
            .reconnect_codes(vec![1001, 1012])
            .reconnect_delay("1s")
            .reconnect_jitter(0.5)
            .reconnect_max_attempts(10)
            .reconnect_max_delay("30s")
            .build()
    }

    fn full_config_json() -> serde_json::Value {
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
    }
}
