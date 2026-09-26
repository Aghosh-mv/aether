# Aether desktop application

Aether is a native desktop application, not a browser implemented inside a web page.

## Platform target

The intended desktop targets are:

- macOS — unsigned development builds are supported; notarization is not part of the development requirement.
- Windows — native executable packaging is planned.
- Linux — native executable packaging is planned.

The engine is Rust-first so that the same document, layout, scripting, networking, privacy, and storage code can be shared across those targets. The current window shell uses portable Rust windowing and framebuffer crates rather than Electron or an embedded system browser.

## UI technology decision

Flutter, Electron, and Swift are not being added merely to make the project look like a desktop app:

- Electron would introduce another browser engine, conflicting with Aether's independent-engine requirement.
- Flutter can be considered later for selected shell UI, but it must not replace the engine or turn Aether into a webview wrapper.
- Swift can provide optional platform interop or a future shell, but AppKit/SwiftUI integrations are platform-specific.

Any future shell must preserve a narrow boundary: the native application controls windows and OS integrations, while the Rust engine controls browser behavior. A compile on one operating system is not treated as proof for the others; each target requires its own build and runtime verification.

## Local run

```sh
cargo run -p aether-browser -- https://example.com
```

This starts the native Aether window directly. It does not start a local web server and does not open Aether inside another browser.
