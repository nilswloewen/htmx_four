use http::StatusCode;
use serde::{Serialize, Serializer};

/// One entry of [`htmx.config.noSwap`](https://four.htmx.org/reference/config/htmx-config-noSwap).
///
/// A status code serializes as a number. A class serializes as `"1xx"` through `"5xx"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusPattern {
    /// One status code, serialized as a number.
    Exact(StatusCode),
    /// `"1xx"`.
    Informational,
    /// `"2xx"`.
    Success,
    /// `"3xx"`.
    Redirection,
    /// `"4xx"`.
    ClientError,
    /// `"5xx"`.
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
