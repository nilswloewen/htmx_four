# htmx_four

Typed Rust values for [htmx 4.0.0](https://github.com/bigskysoftware/htmx/tree/v4.0.0): the [`htmx.config`](https://four.htmx.org/reference/config/htmx-config) object, core [header names](https://four.htmx.org/reference/headers), and the extensions that ship with that release.

The goal is to help Rust users use htmx in its intended way. The types follow the [official docs](https://four.htmx.org/docs), so application code can stay on that path and skip local hacks or workarounds.

`HtmxConfig::to_json` produces the JSON object htmx reads from `<meta name="htmx-config">`.

Header constants are lowercase `&'static str` names, so they work with any web framework and any version of the `http` crate. The [features](#features) add typed `HeaderName` copies.

## Features

No feature is on by default. The default build depends on no `http` crate.

| Feature | Adds | Use it with |
|---|---|---|
| *(none)* | Header names as lowercase `&'static str` constants, for example `request_headers::HX_REQUEST`. | Any framework, and any version of the `http` crate. |
| `http` | A child `http` module in each header module, with the same names as `http::HeaderName` constants, for example `request_headers::http::HX_REQUEST`. Also `From<http::StatusCode>` for `StatusPattern`. | Frameworks on the latest `http` release, such as axum, hyper and reqwest. |

```toml
[dependencies]
htmx_four = { version = "0.1.1", features = ["http"] }
```

The features are additive. The `&str` constants are always present, and `http` only adds the typed copies beside them.

### The `http` feature tracks the latest `http`

The `http` feature always means the latest major version of the [`http`](https://docs.rs/http) crate, which is 1.x today. When `http` releases a new major version, this crate moves the feature to it and releases a new breaking version of `htmx_four`. There is no feature for an older `http` major.

A framework on an older `http` takes the `&str` names. For example, Actix Web 4 uses `http` 0.2:

- Pass the name directly, as in `.insert_header((HX_REDIRECT, path))` or `headers.get(HX_REQUEST)`.
- Make a typed name with `HeaderName::from_static(HX_TRIGGER)` where an API needs one.

## Docs

- [Documentation](https://four.htmx.org/docs)
- [Attributes](https://four.htmx.org/reference/attributes)
- [Headers](https://four.htmx.org/reference/headers)
- [Events](https://four.htmx.org/reference/events)
- [Config](https://four.htmx.org/reference/config)
- [Extensions](https://four.htmx.org/extensions)
- [`htmax.js`](https://four.htmx.org/docs/htmax), the bundle of htmx plus the common extensions

[`SwapStyle`](https://four.htmx.org/reference/attributes/hx-swap) covers the built-in swap styles, including `upsert` and `download`.

## Extensions

This crate supports every [official extension](https://four.htmx.org/extensions) that ships with htmx 4.0.0.

`Extension` is the allow-list for [`htmx.config.extensions`](https://four.htmx.org/reference/config/htmx-config-extensions). Variant names follow the script file in htmx `src/ext`. The allow-list name is the string that file passes to `htmx.registerExtension`, and it is not always the file name. `Extension::Custom` covers any other name.

These extensions also have a typed config section on `HtmxConfig`: `compat`, `historyCache`, `live`, `multipart`, `preload`, `sse`, and `ws`. `hx-browser-indicator` is the top-level `boostBrowserIndicator` field, and `hx-csp` is `safeEval`. Extension header names live under `ext`.

```rust
use htmx_four::{Extension, HtmxConfig, SseConfig, SwapStyle};

fn main() -> serde_json::Result<()> {
    let config = HtmxConfig::builder()
        .default_swap(SwapStyle::InnerHtml)
        .extensions(vec![Extension::HxSse, Extension::HxPreload])
        .sse(SseConfig::builder().reconnect(true).build())
        .build();

    let json = config.to_json()?;
    assert_eq!(
        json,
        r#"{"defaultSwap":"innerHTML","extensions":"sse, preload","sse":{"reconnect":true}}"#
    );
    Ok(())
}
```

Built by htmx CEO [nilswloewen](https://htmx.ceo/) and his trusty clanka's.

![Official CEO of htmx](assets/nils_ceo.jpeg)
