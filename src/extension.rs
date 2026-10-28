use serde::{Serialize, Serializer};
use strum::IntoStaticStr;

/// A script in htmx 4 `src/ext`. The variant name follows that file.
/// `as_str` is the string the file passes to `htmx.registerExtension`, which is what
/// `htmx.config.extensions` checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, IntoStaticStr)]
pub enum Extension {
    #[strum(serialize = "compat")]
    Htmx2Compat,
    #[strum(serialize = "alpine-compat")]
    HxAlpineCompat,
    #[strum(serialize = "browser-indicator")]
    HxBrowserIndicator,
    #[strum(serialize = "hx-csp")]
    HxCsp,
    #[strum(serialize = "download")]
    HxDownload,
    #[strum(serialize = "hx-head")]
    HxHead,
    #[strum(serialize = "history-cache")]
    HxHistoryCache,
    #[strum(serialize = "hx-live")]
    HxLive,
    #[strum(serialize = "hx-multipart")]
    HxMultipart,
    #[strum(serialize = "hx-pending")]
    HxPending,
    #[strum(serialize = "preload")]
    HxPreload,
    #[strum(serialize = "hx-prompt")]
    HxPrompt,
    #[strum(serialize = "ptag")]
    HxPtag,
    #[strum(serialize = "sse")]
    HxSse,
    #[strum(serialize = "hx-targets")]
    HxTargets,
    #[strum(serialize = "upsert")]
    HxUpsert,
    #[strum(serialize = "ws")]
    HxWs,
    #[strum(transparent)]
    Custom(&'static str),
}

impl Extension {
    pub fn as_str(self) -> &'static str {
        self.into()
    }
}

pub(crate) fn serialize_extensions<S>(extensions: &Option<Vec<Extension>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let names = extensions.iter().flatten().map(|extension| extension.as_str()).collect::<Vec<_>>();
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
