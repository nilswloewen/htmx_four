use crate::ext::hx_sse::SseReleaseOn;
use serde::Serialize;
use typed_builder::TypedBuilder;

/// The `sse` section of the htmx config, read by the `hx-sse` extension.
#[derive(Clone, Debug, Default, PartialEq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SseConfig {
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_on_background: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconnect: Option<bool>,

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

    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_on: Option<SseReleaseOn>,
}
