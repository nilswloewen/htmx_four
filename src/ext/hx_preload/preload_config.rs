use serde::Serialize;
use typed_builder::TypedBuilder;

/// The `preload` section of the htmx config, read by the `hx-preload` extension.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PreloadConfig {
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_boost: Option<bool>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_event: Option<String>,

    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_timeout: Option<String>,
}
