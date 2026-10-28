use bon::Builder;
use serde::Serialize;

/// The `preload` section of the htmx config, read by the `hx-preload` extension.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct PreloadConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_boost: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_event: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_timeout: Option<String>,
}
