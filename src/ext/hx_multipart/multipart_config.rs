use bon::Builder;
use serde::Serialize;

/// The `multipart` section of the htmx config, read by the `hx-multipart` extension.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct MultipartConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_on_background: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_delay: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_jitter: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_max_attempts: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect_max_delay: Option<String>,
}
