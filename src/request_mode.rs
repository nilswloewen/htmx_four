use serde::Serialize;

/// A value of [`htmx.config.mode`](https://four.htmx.org/reference/config/htmx-config-mode), passed to `fetch()`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RequestMode {
    /// `"same-origin"`.
    SameOrigin,
    /// `"cors"`.
    Cors,
    /// `"no-cors"`.
    NoCors,
    /// `"navigate"`.
    Navigate,
}
