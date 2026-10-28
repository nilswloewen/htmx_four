use serde::{Serialize, Serializer};

/// A value of [`htmx.config.history`](https://four.htmx.org/reference/config/htmx-config-history).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum History {
    /// `true`: push URLs, and restore them with a request.
    Enabled,
    /// `false`: leave browser history alone.
    Disabled,
    /// `"reload"`: push URLs, and reload the page on back and forward.
    Reload,
}

impl Serialize for History {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_bool(true),
            Self::Disabled => serializer.serialize_bool(false),
            Self::Reload => serializer.serialize_str("reload"),
        }
    }
}
