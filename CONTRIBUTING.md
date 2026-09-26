# Contributing

Keep subsystem boundaries explicit. Add a focused regression test with every behavior change, run `cargo fmt --check` and `cargo test --workspace`, and document architecture changes in `docs/adr/`. Do not claim a website works from a successful load alone.

Pull requests must pass the GitHub Actions quality gates: formatting, workspace tests, clippy with warnings denied, and a release build.
