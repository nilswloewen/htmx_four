use http::HeaderName;

pub const HX_BOOSTED: HeaderName = HeaderName::from_static("hx-boosted");
pub const HX_CURRENT_URL: HeaderName = HeaderName::from_static("hx-current-url");
pub const HX_HISTORY_RESTORE_REQUEST: HeaderName =
    HeaderName::from_static("hx-history-restore-request");
pub const HX_PROMPT: HeaderName = HeaderName::from_static("hx-prompt");
pub const HX_REQUEST: HeaderName = HeaderName::from_static("hx-request");
pub const HX_REQUEST_TYPE: HeaderName = HeaderName::from_static("hx-request-type");
pub const HX_SOURCE: HeaderName = HeaderName::from_static("hx-source");
pub const HX_TARGET: HeaderName = HeaderName::from_static("hx-target");
pub const HX_TRIGGER: HeaderName = HeaderName::from_static("hx-trigger");
pub const HX_TRIGGER_NAME: HeaderName = HeaderName::from_static("hx-trigger-name");
