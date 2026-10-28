use serde::Serialize;
use typed_builder::TypedBuilder;

/// The `ws` section of the htmx config, read by the `hx-ws` extension.
#[derive(Clone, Debug, Default, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct WsConfig {
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_outgoing_messages_queue_size: Option<u32>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_on_background: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<Vec<String>>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_codes: Option<Vec<u16>>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_delay: Option<String>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_jitter: Option<f64>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_max_attempts: Option<u32>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_max_delay: Option<String>,
}
