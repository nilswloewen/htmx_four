# htmx_four

Typed Rust values for [htmx 4](https://four.htmx.org/docs): the [`htmx.config`](https://four.htmx.org/reference/config/htmx-config) object, core [header names](https://four.htmx.org/reference/headers), and the extensions that ship with htmx.

`HtmxConfig::to_json` produces the JSON object htmx reads from `<meta name="htmx-config">`. Header constants are lowercase so they work with `http::HeaderName::from_static`.

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

`Extension` is the allow-list for [`htmx.config.extensions`](https://four.htmx.org/reference/config/htmx-config-extensions). Variant names follow the script file in htmx `src/ext`. The allow-list name is the string that file passes to `htmx.registerExtension`, and it is not always the file name. `Extension::Custom` covers any other name.

| Extension | Variant | Allow-list name | What it does |
| --- | --- | --- | --- |
| [`hx-multipart`](https://four.htmx.org/extensions/hx-multipart) | `HxMultipart` | `hx-multipart` | Stream HTML with `multipart/mixed` |
| [`hx-sse`](https://four.htmx.org/extensions/hx-sse) | `HxSse` | `sse` | Stream HTML with `text/event-stream` (SSE) |
| [`hx-ws`](https://four.htmx.org/extensions/hx-ws) | `HxWs` | `ws` | Stream HTML and send data over WebSockets |
| [`hx-live`](https://four.htmx.org/extensions/hx-live) | `HxLive` | `hx-live` | Add reactive bindings to HTML |
| [`hx-pending`](https://four.htmx.org/extensions/hx-pending) | `HxPending` | `hx-pending` | Show custom content during requests |
| [`hx-browser-indicator`](https://four.htmx.org/extensions/hx-browser-indicator) | `HxBrowserIndicator` | `browser-indicator` | Show tab's spinner with `hx-browser-indicator` |
| [`hx-prompt`](https://four.htmx.org/extensions/hx-prompt) | `HxPrompt` | `hx-prompt` | Prompt before requests with `hx-prompt='Reason?'` |
| [`hx-preload`](https://four.htmx.org/extensions/hx-preload) | `HxPreload` | `preload` | Preload on hover with `hx-preload='mouseover'` |
| [`hx-ptag`](https://four.htmx.org/extensions/hx-ptag) | `HxPtag` | `ptag` | Skip unchanged polls with `HX-PTag: "v42"` |
| [`hx-history-cache`](https://four.htmx.org/extensions/hx-history-cache) | `HxHistoryCache` | `history-cache` | Restore back/forward pages from `sessionStorage` |
| [`hx-head`](https://four.htmx.org/extensions/hx-head) | `HxHead` | `hx-head` | Merge `<head>` tags with `hx-head='merge'` |
| [`hx-upsert`](https://four.htmx.org/extensions/hx-upsert) | `HxUpsert` | `upsert` | Update or insert elements with `hx-swap='upsert'` |
| [`hx-targets`](https://four.htmx.org/extensions/hx-targets) | `HxTargets` | `hx-targets` | Target many elements with `hx-targets='.selector'` |
| [`hx-download`](https://four.htmx.org/extensions/hx-download) | `HxDownload` | `download` | Download files with `hx-swap='download'` |
| [`htmx-2-compat`](https://four.htmx.org/extensions/htmx-2-compat) | `Htmx2Compat` | `compat` | Restore htmx 2.x defaults and event names on htmx 4 |
| [`hx-alpine-compat`](https://four.htmx.org/extensions/hx-alpine-compat) | `HxAlpineCompat` | `alpine-compat` | Run htmx alongside Alpine.js without conflicts |
| [`hx-csp`](https://four.htmx.org/extensions/hx-csp) | `HxCsp` | `hx-csp` | Make htmx work under strict Content Security Policy |

These extensions also have a typed config section on `HtmxConfig`: `compat`, `historyCache`, `live`, `multipart`, `preload`, `sse`, and `ws`. `hx-browser-indicator` is the top-level `boostBrowserIndicator` field, and `hx-csp` is `safeEval`. Extension header names live under `ext`.

```rust
use htmx_four::{Extension, HtmxConfig, SseConfig, SwapStyle};

let config = HtmxConfig::builder()
    .default_swap(SwapStyle::InnerHtml)
    .extensions(vec![Extension::HxSse, Extension::HxPreload])
    .sse(SseConfig::builder().reconnect(true).build())
    .build();

let json = config.to_json().unwrap();
// {"defaultSwap":"innerHTML","extensions":"sse, preload","sse":{"reconnect":true}}
```
