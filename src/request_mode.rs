use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RequestMode {
    SameOrigin,
    Cors,
    NoCors,
    Navigate,
}
