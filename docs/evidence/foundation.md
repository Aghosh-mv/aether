# Foundation Evidence — 2026-09-26

## Verified

- `cargo fmt --all -- --check` passes.
- `cargo test --workspace` passes: HTML parser, DOM text extraction, and layout tests pass; all doc tests pass.
- `cargo run -p aether-browser -- https://example.com` builds and launches the native event loop. The process was stopped manually after launch.
- The repository contains no Chromium, Electron, WebKit, Gecko, or complete browser-engine dependency.

## Not yet verified

- Pixel-level visual output in a native window.
- WHATWG HTML conformance.
- CSS styling, images, fonts, JavaScript, interaction, scrolling, or modern web application workflows.
- Sandbox, site isolation, permissions, storage, downloads, media, extensions, DevTools, or compatibility results.

The current implementation is therefore an early vertical slice, not a normal-browser release.

