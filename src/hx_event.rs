//! One [`HX-Trigger`](https://four.htmx.org/reference/headers/HX-Trigger) value.
//!
//! A header that does not start with `{` is a comma-separated list of event names. A header that
//! starts with `{` is a JSON object whose keys are event names and whose values are event details.
//! htmx 4 fires these events after the swap.

use serde::Serialize;
use serde_json::Value;

/// One event inside an [`HX-Trigger`](https://four.htmx.org/reference/headers/HX-Trigger) header.
#[derive(Clone, Debug, PartialEq)]
pub struct HxEvent {
    name: String,
    detail: Option<Value>,
}

impl HxEvent {
    /// An event with no detail.
    ///
    /// # Errors
    ///
    /// Returns [`HxEventError::Name`] when `name` is empty, has surrounding whitespace, contains
    /// a comma, starts with `{`, or contains an ASCII control character.
    pub fn name(name: &str) -> Result<Self, HxEventError> {
        if !is_token(name) {
            return Err(HxEventError::Name);
        }
        Ok(Self {
            name: name.to_owned(),
            detail: None,
        })
    }

    /// An event whose [`detail`](https://four.htmx.org/reference/headers/HX-Trigger#event-detail) is `detail`.
    ///
    /// A string is one JSON string. htmx exposes it as `event.detail.value`. An object is copied
    /// onto the event detail field by field.
    ///
    /// # Errors
    ///
    /// Returns [`HxEventError::Name`] for the same names [`name`](Self::name) rejects.
    pub fn with_detail(name: &str, detail: &Value) -> Result<Self, HxEventError> {
        if !is_token(name) {
            return Err(HxEventError::Name);
        }
        Ok(Self {
            name: name.to_owned(),
            detail: Some(detail.clone()),
        })
    }

    /// An event whose detail is `detail` serialized as JSON.
    ///
    /// # Errors
    ///
    /// Returns [`HxEventError::Name`] for the same names [`name`](Self::name) rejects.
    /// Returns [`HxEventError::Detail`] when `detail` does not serialize.
    pub fn try_detail<T>(name: &str, detail: &T) -> Result<Self, HxEventError>
    where
        T: Serialize,
    {
        let detail = serde_json::to_value(detail).map_err(|_| HxEventError::Detail)?;
        Self::with_detail(name, &detail)
    }

    /// The event name.
    #[must_use]
    pub fn event_name(&self) -> &str {
        &self.name
    }

    /// The JSON detail, when this event has one.
    #[must_use]
    pub fn detail(&self) -> Option<&Value> {
        self.detail.as_ref()
    }
}

/// Why an [`HxEvent`] could not be built.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HxEventError {
    /// The event name is not one token.
    Name,
    /// The detail value did not serialize as JSON.
    Detail,
}

impl std::fmt::Display for HxEventError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Name => "invalid hx-trigger event name",
            Self::Detail => "hx-trigger detail is not json",
        })
    }
}

impl std::error::Error for HxEventError {}

/// The events in one [`HX-Trigger`](https://four.htmx.org/reference/headers/HX-Trigger) header.
///
/// Names stay in insertion order. A later event with the same name does not replace the first.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HxTrigger {
    events: Vec<HxEvent>,
}

impl HxTrigger {
    /// An empty header.
    #[must_use]
    pub const fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Reads a header value.
    ///
    /// A value that starts with `{` is JSON. Anything else is a comma-separated name list.
    /// Surrounding whitespace around each name is removed. Empty names are skipped.
    ///
    /// # Errors
    ///
    /// Returns [`HxTriggerParseError::Json`] when a value starts with `{` and is not a JSON object.
    pub fn parse(value: &str) -> Result<Self, HxTriggerParseError> {
        if value.starts_with('{') {
            parse_json(value)
        } else {
            Ok(parse_names(value))
        }
    }

    /// The events in insertion order.
    #[must_use]
    pub fn events(&self) -> &[HxEvent] {
        &self.events
    }

    /// Adds `event` when its name is not already present.
    ///
    /// Returns `false` when the name was already there. The earlier detail stays.
    #[must_use = "false means the name was already present"]
    pub fn insert(&mut self, event: HxEvent) -> bool {
        if self
            .events
            .iter()
            .any(|existing| existing.name == event.name)
        {
            return false;
        }
        self.events.push(event);
        true
    }

    /// The header text htmx 4 reads.
    ///
    /// Bare names become `event1, event2`. Any detail, or a name that is not a single token,
    /// becomes one JSON object. A bare name inside that object is `true`.
    ///
    /// # Errors
    ///
    /// Returns an error when a detail value cannot be written as JSON.
    pub fn to_header_value(&self) -> Result<String, serde_json::Error> {
        if self
            .events
            .iter()
            .all(|event| event.detail.is_none() && is_token(&event.name))
        {
            return Ok(self
                .events
                .iter()
                .map(|event| event.name.as_str())
                .collect::<Vec<_>>()
                .join(", "));
        }

        let mut out = String::from('{');
        for (index, event) in self.events.iter().enumerate() {
            if index > 0 {
                out.push(',');
            }
            out.push_str(&serde_json::to_string(&event.name)?);
            out.push(':');
            match &event.detail {
                Some(detail) => out.push_str(&serde_json::to_string(detail)?),
                None => out.push_str("true"),
            }
        }
        out.push('}');
        Ok(out)
    }
}

/// Why an [`HX-Trigger`](https://four.htmx.org/reference/headers/HX-Trigger) value could not be read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HxTriggerParseError {
    /// The value starts with `{` and is not a JSON object.
    Json,
}

impl std::fmt::Display for HxTriggerParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("hx-trigger json value is not an object")
    }
}

impl std::error::Error for HxTriggerParseError {}

fn parse_names(value: &str) -> HxTrigger {
    let events = value
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(|name| HxEvent {
            name: name.to_owned(),
            detail: None,
        })
        .collect();
    HxTrigger { events }
}

fn parse_json(value: &str) -> Result<HxTrigger, HxTriggerParseError> {
    let parsed: Value = serde_json::from_str(value).map_err(|_| HxTriggerParseError::Json)?;
    let Some(object) = parsed.as_object() else {
        return Err(HxTriggerParseError::Json);
    };
    let events = object
        .iter()
        .map(|(name, detail)| HxEvent {
            name: name.clone(),
            detail: Some(detail.clone()),
        })
        .collect();
    Ok(HxTrigger { events })
}

fn is_token(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with(char::is_whitespace)
        && !name.ends_with(char::is_whitespace)
        && !name.contains(',')
        && !name.starts_with('{')
        && !name.chars().any(|character| character.is_ascii_control())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn bare_names_join_with_a_comma_and_a_space() {
        let mut trigger = HxTrigger::new();
        assert!(trigger.insert(HxEvent::name("saved").unwrap()));
        assert!(trigger.insert(HxEvent::name("refresh").unwrap()));
        assert!(!trigger.insert(HxEvent::name("saved").unwrap()));
        assert_eq!(trigger.to_header_value().unwrap(), "saved, refresh");
    }

    #[test]
    fn a_detail_switches_the_header_to_json() {
        let mut trigger = HxTrigger::new();
        assert!(trigger.insert(HxEvent::name("refresh").unwrap()));
        assert!(
            trigger.insert(HxEvent::try_detail("notification", &json!({"level": "info"})).unwrap())
        );

        let value: Value = serde_json::from_str(&trigger.to_header_value().unwrap()).unwrap();
        assert_eq!(value["refresh"], true);
        assert_eq!(value["notification"]["level"], "info");
    }

    #[test]
    fn a_string_detail_stays_a_string() {
        let event = HxEvent::with_detail("notification", &json!("{not: \"json\"")).unwrap();
        let mut trigger = HxTrigger::new();
        assert!(trigger.insert(event));

        let value: Value = serde_json::from_str(&trigger.to_header_value().unwrap()).unwrap();
        assert_eq!(value["notification"], "{not: \"json\"");
    }

    #[test]
    fn parse_keeps_the_first_detail() {
        let mut trigger = HxTrigger::parse(r#"{"saved":{"id":7}}"#).unwrap();
        assert!(!trigger.insert(HxEvent::name("saved").unwrap()));
        assert_eq!(trigger.events()[0].detail().unwrap()["id"], 7);
    }
}
