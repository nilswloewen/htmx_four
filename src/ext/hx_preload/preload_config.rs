use bon::Builder;
use serde::Serialize;

/// The `preload` section of [`htmx.config`](https://four.htmx.org/reference/config/htmx-config), read by [`hx-preload`](https://four.htmx.org/extensions/hx-preload#configuration).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct PreloadConfig {
    /// `autoBoost`: preload boosted links without an `hx-preload` attribute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_boost: Option<bool>,

    /// `boostEvent`: the event that preloads an auto-boosted link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_event: Option<String>,

    /// `boostTimeout`: how long a preloaded boosted response stays usable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_timeout: Option<String>,
}
