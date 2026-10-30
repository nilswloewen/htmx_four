//! The [`HX-Location`](https://four.htmx.org/reference/headers/HX-Location) response value.
//!
//! A bare path tells htmx to `GET` that path. htmx 4.0.0 then pushes history, which is the same
//! as `htmx.ajax('GET', path, { push: 'true' })`. Any other field is one JSON object. `path` is
//! required either way.
//!
//! The JSON fields are the serializable [`htmx.ajax()`](https://four.htmx.org/reference/methods/htmx-ajax#options)
//! options: `source`, `target`, `swap`, `select`, `selectOOB`, `transition`, `values`, `headers`,
//! `push`, and `replace`. `event` is the DOM event passed to `htmx.ajax()`, so this header does
//! not carry it. htmx 4.0.0 does not read a `handler` option.

use std::collections::BTreeMap;

use bon::Builder;
use serde::Serialize;
use serde_json::Value;

use crate::SwapStyle;

/// A [`push`](https://four.htmx.org/reference/headers/HX-Push-Url) or [`replace`](https://four.htmx.org/reference/headers/HX-Replace-Url) instruction.
///
/// `HX-Location` uses the same two values inside its JSON body. htmx 4.0.0 treats boolean
/// `false` as "leave history alone", which lets `replace` apply.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HxUrlHistory {
    /// Do not update the history entry. The header text is `false`.
    Skip,
    /// A relative or absolute URL. The text `true` means the response URL.
    Url(String),
}

impl HxUrlHistory {
    /// The text of an `HX-Push-Url` or `HX-Replace-Url` header.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Skip => "false",
            Self::Url(url) => url,
        }
    }
}

impl Serialize for HxUrlHistory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Skip => serializer.serialize_bool(false),
            Self::Url(url) => serializer.serialize_str(url),
        }
    }
}

/// One [`HX-Location`](https://four.htmx.org/reference/headers/HX-Location) value.
///
/// Fields left unset are omitted. [`to_header_value`](Self::to_header_value) writes the path
/// alone when every other field is unset, and JSON otherwise.
#[derive(Clone, Debug, PartialEq, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct HxLocation {
    /// The URL htmx requests. This is the `path` field.
    pub path: String,

    /// [`target`](https://four.htmx.org/reference/methods/htmx-ajax#options): the element that receives the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// [`source`](https://four.htmx.org/reference/methods/htmx-ajax#options): the element whose attributes and values apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// [`swap`](https://four.htmx.org/reference/methods/htmx-ajax#options): a serialized `hx-swap` value, including modifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<String>,

    /// [`select`](https://four.htmx.org/reference/methods/htmx-ajax#options): the response fragment to swap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select: Option<String>,

    /// [`selectOOB`](https://four.htmx.org/reference/methods/htmx-ajax#options): out-of-band content from the response.
    #[serde(rename = "selectOOB", skip_serializing_if = "Option::is_none")]
    pub select_oob: Option<String>,

    /// [`transition`](https://four.htmx.org/reference/methods/htmx-ajax#options): use a view transition for the swap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<bool>,

    /// [`values`](https://four.htmx.org/reference/methods/htmx-ajax#options): values added to the follow-up request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Value>,

    /// [`headers`](https://four.htmx.org/reference/methods/htmx-ajax#options): headers added to the follow-up request.
    #[serde(skip_serializing_if = "is_empty_headers")]
    pub headers: Option<BTreeMap<String, String>>,

    /// When `false`, htmx does not push the new path. A string pushes that URL instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push: Option<HxUrlHistory>,

    /// A URL to replace into the current history entry.
    ///
    /// The text `true` replaces that entry with the response path. See
    /// [`HX-Replace-Url`](https://four.htmx.org/reference/headers/HX-Replace-Url).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<HxUrlHistory>,
}

impl HxLocation {
    /// Stores `values` as the JSON object sent with the follow-up request.
    ///
    /// On failure `self.values` is left as it was.
    ///
    /// # Errors
    ///
    /// Returns the serde error when `values` does not serialize to JSON.
    pub fn set_values<T>(&mut self, values: &T) -> Result<(), serde_json::Error>
    where
        T: Serialize,
    {
        self.values = Some(serde_json::to_value(values)?);
        Ok(())
    }

    /// Sets [`swap`](Self::swap) from a [`SwapStyle`]. Modifiers such as `transition:true` need the string field.
    pub fn set_swap_style(&mut self, swap: SwapStyle) {
        self.swap = Some(swap.as_str().to_owned());
    }

    /// The `HX-Location` header text.
    ///
    /// # Errors
    ///
    /// Returns [`HxLocationError::EmptyPath`] when `path` is empty.
    /// Returns [`HxLocationError::Json`] when the JSON object cannot be written.
    pub fn to_header_value(&self) -> Result<String, HxLocationError> {
        if self.path.is_empty() {
            return Err(HxLocationError::EmptyPath);
        }
        if self.path_only() && !self.path.starts_with('{') {
            return Ok(self.path.clone());
        }
        serde_json::to_string(self).map_err(HxLocationError::Json)
    }

    fn path_only(&self) -> bool {
        self.target.is_none()
            && self.source.is_none()
            && self.swap.is_none()
            && self.select.is_none()
            && self.select_oob.is_none()
            && self.transition.is_none()
            && self.values.is_none()
            && is_empty_headers(&self.headers)
            && self.push.is_none()
            && self.replace.is_none()
    }
}

/// Why [`HxLocation::to_header_value`] failed.
#[derive(Debug)]
pub enum HxLocationError {
    /// `path` is empty.
    EmptyPath,
    /// The JSON object could not be written.
    Json(serde_json::Error),
}

impl std::fmt::Display for HxLocationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyPath => formatter.write_str("hx-location path is empty"),
            Self::Json(_) => formatter.write_str("hx-location json failed"),
        }
    }
}

impl std::error::Error for HxLocationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::EmptyPath => None,
        }
    }
}

#[expect(
    clippy::ref_option,
    reason = "serde skip_serializing_if passes a reference to the field, which is already an Option"
)]
fn is_empty_headers(headers: &Option<BTreeMap<String, String>>) -> bool {
    headers.as_ref().is_none_or(BTreeMap::is_empty)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use serde::Serialize;
    use serde_json::Value;

    use crate::SwapStyle;

    #[derive(Serialize)]
    struct Payload<'a> {
        id: u32,
        name: &'a str,
    }

    #[test]
    fn a_path_stays_a_path() {
        let location = HxLocation::builder().path("/dashboard").build();
        assert_eq!(location.to_header_value().unwrap(), "/dashboard");
    }

    #[test]
    fn options_become_one_json_object() {
        let mut location = HxLocation::builder()
            .path("/search")
            .target("#results")
            .select("#matches")
            .select_oob("#extra")
            .headers(BTreeMap::from([("X-Demo".to_owned(), "1".to_owned())]))
            .push(HxUrlHistory::Skip)
            .build();
        location.set_swap_style(SwapStyle::OuterHtml);
        location
            .set_values(&Payload {
                id: 7,
                name: "demo",
            })
            .unwrap();

        let parsed: Value = serde_json::from_str(&location.to_header_value().unwrap()).unwrap();
        assert_eq!(parsed["path"], "/search");
        assert_eq!(parsed["target"], "#results");
        assert_eq!(parsed["select"], "#matches");
        assert_eq!(parsed["selectOOB"], "#extra");
        assert_eq!(parsed["headers"]["X-Demo"], "1");
        assert_eq!(parsed["swap"], "outerHTML");
        assert_eq!(parsed["push"], false);
        assert_eq!(parsed["values"]["id"], 7);
        assert_eq!(parsed["values"]["name"], "demo");
    }

    #[test]
    fn an_empty_path_is_rejected() {
        let location = HxLocation::builder().path("").build();
        let error = location.to_header_value().unwrap_err();
        assert!(matches!(error, HxLocationError::EmptyPath));
        assert_eq!(error.to_string(), "hx-location path is empty");
    }

    #[test]
    fn a_path_that_looks_like_json_is_written_as_json() {
        let location = HxLocation::builder().path("{/dashboard").build();
        let parsed: Value = serde_json::from_str(&location.to_header_value().unwrap()).unwrap();
        assert_eq!(parsed["path"], "{/dashboard");
    }

    struct Fails;

    impl Serialize for Fails {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("no"))
        }
    }

    #[test]
    fn a_failed_value_leaves_the_path_alone() {
        let mut location = HxLocation::builder().path("/dashboard").build();

        assert!(location.set_values(&Fails).is_err());
        assert!(location.values.is_none());
        assert_eq!(location.to_header_value().unwrap(), "/dashboard");
    }
}
