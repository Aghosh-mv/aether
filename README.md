# Aether

Aether is an independent browser-engine project written primarily in Rust. It is not Chromium, Electron, WebKit, Gecko, or a wrapper around another browser.

## Current evidence

The repository currently contains the first foundation slice: a small HTML parser and DOM, block-text layout, CPU framebuffer painting, HTTPS navigation, and a native window. It is an early engineering foundation, not a daily-use browser. CSS, JavaScript, persistent storage, security isolation, media, extensions, DevTools, and real compatibility testing are not complete.

```sh
cargo test --workspace
cargo run -p aether-browser -- https://example.com
```

## Project map

- `engine/dom` — owned DOM data model
- `engine/html` — owned tokenizer/tree builder
- `engine/layout` — first block-text layout pass
- `engine/paint` — first CPU display surface
- `browser` — HTTPS navigation and native window integration
- `docs/adr` — architecture decisions
- `.summer/plans` — implementation plans

