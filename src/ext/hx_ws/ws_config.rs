use bon::Builder;
use serde::Serialize;

/// The `ws` section of [`htmx.config`](https://four.htmx.org/reference/config/htmx-config), read by [`hx-ws`](https://four.htmx.org/extensions/hx-ws#config).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct WsConfig {
    /// [`maxOutgoingMessagesQueueSize`](https://four.htmx.org/extensions/hx-ws#wsmaxoutgoingmessagesqueuesize): how many outgoing messages to queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_outgoing_messages_queue_size: Option<u32>,

    /// [`pauseOnBackground`](https://four.htmx.org/extensions/hx-ws#wspauseonbackground): close the socket while the page is hidden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_on_background: Option<bool>,

    /// [`protocols`](https://four.htmx.org/extensions/hx-ws#wsprotocols): WebSocket subprotocols to offer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,

    /// [`reconnect`](https://four.htmx.org/extensions/hx-ws#wsreconnect): reconnect after the socket closes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect: Option<bool>,

    /// [`reconnectCodes`](https://four.htmx.org/extensions/hx-ws#wsreconnectcodes): close codes that should reconnect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_codes: Option<Vec<u16>>,

    /// [`reconnectDelay`](https://four.htmx.org/extensions/hx-ws#wsreconnectdelay): wait before the first reconnect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_delay: Option<String>,

    /// [`reconnectJitter`](https://four.htmx.org/extensions/hx-ws#wsreconnectjitter): randomize each reconnect delay by this fraction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_jitter: Option<f64>,

    /// [`reconnectMaxAttempts`](https://four.htmx.org/extensions/hx-ws#wsreconnectmaxattempts): how many times to reconnect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_max_attempts: Option<u32>,

    /// [`reconnectMaxDelay`](https://four.htmx.org/extensions/hx-ws#wsreconnectmaxdelay): the longest wait between reconnects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_max_delay: Option<String>,
}
