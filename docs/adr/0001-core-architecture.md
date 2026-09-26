# ADR 0001: Owned browser pipeline with isolated Rust crates

## Decision

Implement Aether's browser pipeline as small Rust crates with explicit boundaries: DOM, HTML, layout, paint, and browser orchestration. Use mature libraries only for isolated infrastructure such as HTTPS transport and native surface access.

## Rationale

This makes each layer testable without a window and prevents a hidden dependency on a complete browser engine. It also permits later replacement of the CPU paint path with a display-list/GPU compositor without rewriting parsing or layout.

## Consequences

The first slice is intentionally limited and cannot render modern websites correctly. Every later capability must be added with tests and evidence rather than status labels or screenshots alone.

