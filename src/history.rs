use serde::{Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum History {
    Enabled,
    Disabled,
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
