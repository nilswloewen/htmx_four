# htmx_four

This crate is a boring library of htmx 4 types: config, header names, and extension settings. It helps Rust users follow htmx the way the official docs describe it. It is the base for later integrations.

Official docs live at https://four.htmx.org. When you document, name, or change a type, cite the page for that config key, header, attribute, or extension. Leave the link out when no such page exists. The htmx 4.0.0 source decides the wire format.

When you write or change Rust, follow the BeautifulRust [`rust-best-practices`](https://github.com/morganlinton/BeautifulRust/tree/main/skills/rust-best-practices) skill. Use [`rust-review`](https://github.com/morganlinton/BeautifulRust/tree/main/skills/rust-review) when asked to review. Keep config fields public, keep the enums exhaustive, and keep Clippy at the default `cargo clippy --all-targets -- -D warnings` bar.

Put an Actix or Axum integration in its own crate. Leave this crate without a web framework.
