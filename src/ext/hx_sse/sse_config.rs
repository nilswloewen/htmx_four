use crate::ext::hx_sse::SseReleaseOn;
use bon::Builder;
use serde::Serialize;

/// The `sse` section of [`htmx.config`](https://four.htmx.org/reference/config/htmx-config), read by [`hx-sse`](https://four.htmx.org/extensions/hx-sse#config).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct SseConfig {
    /// [`pauseOnBackground`](https://four.htmx.org/extensions/hx-sse#ssepauseonbackground): close the stream while the page is hidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_on_background: Option<bool>,

    /// [`reconnect`](https://four.htmx.org/extensions/hx-sse#ssereconnect): reconnect after the stream closes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect: Option<bool>,

    /// [`reconnectDelay`](https://four.htmx.org/extensions/hx-sse#ssereconnectdelay): wait before the first reconnect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_delay: Option<String>,

    /// [`reconnectJitter`](https://four.htmx.org/extensions/hx-sse#ssereconnectjitter): randomize each reconnect delay by this fraction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_jitter: Option<f64>,

    /// [`reconnectMaxAttempts`](https://four.htmx.org/extensions/hx-sse#ssereconnectmaxattempts): how many times to reconnect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_max_attempts: Option<u32>,

    /// [`reconnectMaxDelay`](https://four.htmx.org/extensions/hx-sse#ssereconnectmaxdelay): the longest wait between reconnects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_max_delay: Option<String>,

    /// [`releaseOn`](https://four.htmx.org/extensions/hx-sse#ssereleaseon): when indicators hide and elements are enabled again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_on: Option<SseReleaseOn>,
}
