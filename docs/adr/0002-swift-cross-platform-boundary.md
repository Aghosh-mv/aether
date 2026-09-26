# ADR 0002: Swift interoperability without replacing the Rust engine

## Decision

Aether remains primarily Rust. Swift may be used for optional platform shells, macOS integrations, or a future cross-platform application layer, but it is not a reason to replace the current engine or claim universal native UI support.

The stable boundary is a C-compatible platform interface or generated bindings around narrow services such as window creation, secure storage, notifications, and accessibility. The engine remains independent of Apple frameworks.

## Cross-platform reality

Swift supports Linux and Windows toolchains, but AppKit, SwiftUI, Keychain, and Apple notarization are Apple-specific. Linux and Windows require separate windowing, credential, notification, installer, and default-browser integrations, or a cross-platform library. A Swift build targeting Linux or Windows does not require Apple notarization.

## Consequences

- Rust continues to own HTML, DOM, CSS, layout, paint, networking, JavaScript, and security-sensitive core logic.
- A future Swift shell can be shipped independently on platforms where it is useful.
- Every platform integration must be built and tested on its target OS; compiling Swift on macOS does not prove Linux or Windows behavior.
- Aether can ship unsigned developer builds or platform-native signing later without making notarization a requirement for non-Apple targets.
