use bon::Builder;
use serde::Serialize;

/// The `multipart` section of [`htmx.config`](https://four.htmx.org/reference/config/htmx-config), read by [`hx-multipart`](https://four.htmx.org/extensions/hx-multipart#config).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct MultipartConfig {
    /// [`pauseOnBackground`](https://four.htmx.org/extensions/hx-multipart#multipartpauseonbackground): close the stream while the page is hidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_on_background: Option<bool>,

    /// [`reconnect`](https://four.htmx.org/extensions/hx-multipart#multipartreconnect): reconnect after the stream closes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect: Option<bool>,

    /// [`reconnectDelay`](https://four.htmx.org/extensions/hx-multipart#multipartreconnectdelay): wait before the first reconnect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_delay: Option<String>,

    /// [`reconnectJitter`](https://four.htmx.org/extensions/hx-multipart#multipartreconnectjitter): randomize each reconnect delay by this fraction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_jitter: Option<f64>,

    /// [`reconnectMaxAttempts`](https://four.htmx.org/extensions/hx-multipart#multipartreconnectmaxattempts): how many times to reconnect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_max_attempts: Option<u32>,

    /// [`reconnectMaxDelay`](https://four.htmx.org/extensions/hx-multipart#multipartreconnectmaxdelay): the longest wait between reconnects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_max_delay: Option<String>,
}
