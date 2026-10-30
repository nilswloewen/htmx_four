# `htmx_four`

Typed Rust values for [htmx 4.0.0](https://github.com/bigskysoftware/htmx/tree/v4.0.0): the [`htmx.config`](https://four.htmx.org/reference/config/htmx-config) object, core [header names](https://four.htmx.org/reference/headers), and the extensions that ship with that release.

The goal is to help Rust users use htmx in its intended way. The types follow the [official docs](https://four.htmx.org/docs), so application code can stay on that path and skip local hacks or workarounds.

This crate is htmx 4 and newer only. It is not backwards compatible with htmx 2. htmx 4 can load [`htmx-2-compat`](https://four.htmx.org/extensions/htmx-2-compat) so an htmx 2 page keeps running, and `CompatConfig` types that extension. The extension does not restore the header names htmx 4 dropped. The Cargo feature of the same name exports those constants:

```toml
htmx_four = { version = "0.1.4", features = ["htmx-2-compat"] }
```

With that feature the crate also exports the htmx 2 request headers `HX-Trigger` and `HX-Trigger-Name`, and the htmx 2 response headers `HX-Push`, `HX-Trigger-After-Swap`, and `HX-Trigger-After-Settle`. htmx 4 sends [`HX-Source`](https://four.htmx.org/reference/headers/HX-Source) instead of the request `HX-Trigger` headers, and it reads [`HX-Push-Url`](https://four.htmx.org/reference/headers/HX-Push-Url) and [`HX-Trigger`](https://four.htmx.org/reference/headers/HX-Trigger). See the [header changes in htmx 4](https://four.htmx.org/docs/whats-new-in-htmx-4#request-headers).

`HtmxConfig::to_json` produces the JSON object htmx reads from `<meta name="htmx-config">`.

Header constants are lowercase `&'static str` names, for example `request_headers::HX_REQUEST`. A framework crate turns a name into a `HeaderName` with the `http` type that framework re-exports. Actix Web 4 re-exports `http` 0.2. Axum re-exports `http` 1.

```toml
[dependencies]
htmx_four = "0.1.4"
```

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
