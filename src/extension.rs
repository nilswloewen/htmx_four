use serde::{Serialize, Serializer};
use strum::IntoStaticStr;

/// A script in htmx 4 [`src/ext`](https://four.htmx.org/extensions). The variant name follows that file.
///
/// [`as_str`](Self::as_str) is the string the file passes to `htmx.registerExtension`, which is what
/// [`htmx.config.extensions`](https://four.htmx.org/reference/config/htmx-config-extensions) checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoStaticStr)]
pub enum Extension {
    /// [`htmx-2-compat`](https://four.htmx.org/extensions/htmx-2-compat), registered as `compat`.
    #[strum(serialize = "compat")]
    Htmx2Compat,
    /// [`hx-alpine-compat`](https://four.htmx.org/extensions/hx-alpine-compat), registered as `alpine-compat`.
    #[strum(serialize = "alpine-compat")]
    HxAlpineCompat,
    /// [`hx-browser-indicator`](https://four.htmx.org/extensions/hx-browser-indicator), registered as `browser-indicator`.
    #[strum(serialize = "browser-indicator")]
    HxBrowserIndicator,
    /// [`hx-csp`](https://four.htmx.org/extensions/hx-csp), registered as `hx-csp`.
    #[strum(serialize = "hx-csp")]
    HxCsp,
    /// [`hx-download`](https://four.htmx.org/extensions/hx-download), registered as `download`.
    #[strum(serialize = "download")]
    HxDownload,
    /// [`hx-head`](https://four.htmx.org/extensions/hx-head), registered as `hx-head`.
    #[strum(serialize = "hx-head")]
    HxHead,
    /// [`hx-history-cache`](https://four.htmx.org/extensions/hx-history-cache), registered as `history-cache`.
    #[strum(serialize = "history-cache")]
    HxHistoryCache,
    /// [`hx-live`](https://four.htmx.org/extensions/hx-live), registered as `hx-live`.
    #[strum(serialize = "hx-live")]
    HxLive,
    /// [`hx-multipart`](https://four.htmx.org/extensions/hx-multipart), registered as `hx-multipart`.
    #[strum(serialize = "hx-multipart")]
    HxMultipart,
    /// [`hx-pending`](https://four.htmx.org/extensions/hx-pending), registered as `hx-pending`.
    #[strum(serialize = "hx-pending")]
    HxPending,
    /// [`hx-preload`](https://four.htmx.org/extensions/hx-preload), registered as `preload`.
    #[strum(serialize = "preload")]
    HxPreload,
    /// [`hx-prompt`](https://four.htmx.org/extensions/hx-prompt), registered as `hx-prompt`.
    #[strum(serialize = "hx-prompt")]
    HxPrompt,
    /// [`hx-ptag`](https://four.htmx.org/extensions/hx-ptag), registered as `ptag`.
    #[strum(serialize = "ptag")]
    HxPtag,
    /// [`hx-sse`](https://four.htmx.org/extensions/hx-sse), registered as `sse`.
    #[strum(serialize = "sse")]
    HxSse,
    /// [`hx-targets`](https://four.htmx.org/extensions/hx-targets), registered as `hx-targets`.
    #[strum(serialize = "hx-targets")]
    HxTargets,
    /// [`hx-upsert`](https://four.htmx.org/extensions/hx-upsert), registered as `upsert`.
    #[strum(serialize = "upsert")]
    HxUpsert,
    /// [`hx-ws`](https://four.htmx.org/extensions/hx-ws), registered as `ws`.
    #[strum(serialize = "ws")]
    HxWs,
    /// A `registerExtension` name that is not one of the scripts shipped with htmx 4.0.0.
    #[strum(transparent)]
    Custom(&'static str),
}

impl Extension {
    /// The name passed to `htmx.registerExtension`.
    ///
    /// [`htmx.config.extensions`](https://four.htmx.org/reference/config/htmx-config-extensions) matches this string.
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

pub(crate) fn serialize_extensions<S>(
    extensions: &Option<Vec<Extension>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let names = extensions
        .iter()
        .flatten()
        .map(|extension| extension.as_str())
        .collect::<Vec<_>>();
    names.join(", ").serialize(serializer)
}

#[cfg(test)]
mod tests {
    use super::Extension;

    #[test]
    fn register_names_match_v4_0_0_scripts() {
        assert_eq!(Extension::Htmx2Compat.as_str(), "compat");
        assert_eq!(Extension::HxAlpineCompat.as_str(), "alpine-compat");
        assert_eq!(Extension::HxBrowserIndicator.as_str(), "browser-indicator");
        assert_eq!(Extension::HxCsp.as_str(), "hx-csp");
        assert_eq!(Extension::HxDownload.as_str(), "download");
        assert_eq!(Extension::HxHead.as_str(), "hx-head");
        assert_eq!(Extension::HxHistoryCache.as_str(), "history-cache");
        assert_eq!(Extension::HxLive.as_str(), "hx-live");
        assert_eq!(Extension::HxMultipart.as_str(), "hx-multipart");
        assert_eq!(Extension::HxPending.as_str(), "hx-pending");
        assert_eq!(Extension::HxPreload.as_str(), "preload");
        assert_eq!(Extension::HxPrompt.as_str(), "hx-prompt");
        assert_eq!(Extension::HxPtag.as_str(), "ptag");
        assert_eq!(Extension::HxSse.as_str(), "sse");
        assert_eq!(Extension::HxTargets.as_str(), "hx-targets");
        assert_eq!(Extension::HxUpsert.as_str(), "upsert");
        assert_eq!(Extension::HxWs.as_str(), "ws");
    }
}
