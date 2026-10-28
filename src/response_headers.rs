use http::HeaderName;

pub const HX_LOCATION: HeaderName = HeaderName::from_static("hx-location");
pub const HX_PUSH: HeaderName = HeaderName::from_static("hx-push");
pub const HX_PUSH_URL: HeaderName = HeaderName::from_static("hx-push-url");
pub const HX_REDIRECT: HeaderName = HeaderName::from_static("hx-redirect");
pub const HX_REFRESH: HeaderName = HeaderName::from_static("hx-refresh");
pub const HX_REPLACE_URL: HeaderName = HeaderName::from_static("hx-replace-url");
pub const HX_RESELECT: HeaderName = HeaderName::from_static("hx-reselect");
pub const HX_RESWAP: HeaderName = HeaderName::from_static("hx-reswap");
pub const HX_RETARGET: HeaderName = HeaderName::from_static("hx-retarget");
pub const HX_TRIGGER: HeaderName = HeaderName::from_static("hx-trigger");
pub const HX_TRIGGER_AFTER_SETTLE: HeaderName = HeaderName::from_static("hx-trigger-after-settle");
pub const HX_TRIGGER_AFTER_SWAP: HeaderName = HeaderName::from_static("hx-trigger-after-swap");
