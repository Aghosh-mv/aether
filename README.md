# Aether

Aether is an independent cross-platform desktop browser application written primarily in Rust. It is not a website, web app, Chromium, Electron, WebKit, Gecko, or a wrapper around another browser. The current `aether-browser` binary opens a native desktop window and is intended to run on macOS, Windows, and Linux.

Swift is an optional future platform-shell/interoperability layer; see [ADR 0002](docs/adr/0002-swift-cross-platform-boundary.md). The current engine remains Rust so its core is not coupled to Apple-only frameworks or notarization.

## License

Aether is released under the custom [Aether Reciprocal Source License](LICENSE). It requires modified source portions to be shared with `aghoshpratheesh@gmail.com`, while allowing modified works to remain private. This is a source-available custom license, not an OSI-approved Open Source license.

## Desktop application boundary

Aether is shipped as an executable application. A website may eventually be one of the things Aether visits, but Aether itself does not run inside a website. The browser shell owns windows, tabs, navigation, profiles, privacy state, downloads, settings, and other desktop behavior; the engine owns document parsing, styling, layout, painting, scripting, and networking.

The current shell uses Rust and `winit`/`softbuffer` for a portable native window. Flutter, Electron, and Swift remain optional integration choices, not hidden runtime dependencies. This keeps the core independent of Apple frameworks and avoids making Apple notarization a requirement. macOS users may run an unsigned development build subject to macOS Gatekeeper policy; Windows and Linux builds do not require Apple notarization.

## Current evidence

The repository currently contains the first foundation slice: a small HTML parser and DOM, block-text layout, CPU framebuffer painting, HTTPS navigation, and a native window. It is an early engineering foundation, not a daily-use browser. CSS, JavaScript, persistent storage, security isolation, media, extensions, DevTools, and real compatibility testing are not complete.

```sh
cargo test --workspace
cargo run -p aether-browser -- https://example.com
```

## Project map

- `engine/dom` — owned DOM data model
- `engine/html` — owned tokenizer/tree builder
- `engine/css` — first stylesheet parser and computed style values
- `engine/layout` — first style-aware block-text layout pass
- `engine/javascript` — early AetherJS lexer/parser/evaluator seed
- `engine/paint` — first CPU display surface
- `browser` — HTTPS navigation and native window integration
- `docs/adr` — architecture decisions
- `.summer/plans` — implementation plans
