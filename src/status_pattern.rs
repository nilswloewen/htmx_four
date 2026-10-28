use http::StatusCode;
use serde::{Serialize, Serializer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusPattern {
    Exact(StatusCode),
    Informational,
    Success,
    Redirection,
    ClientError,
    ServerError,
}

impl Serialize for StatusPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Exact(status) => serializer.serialize_u16(status.as_u16()),
            Self::Informational => serializer.serialize_str("1xx"),
            Self::Success => serializer.serialize_str("2xx"),
            Self::Redirection => serializer.serialize_str("3xx"),
            Self::ClientError => serializer.serialize_str("4xx"),
            Self::ServerError => serializer.serialize_str("5xx"),
        }
    }
}
