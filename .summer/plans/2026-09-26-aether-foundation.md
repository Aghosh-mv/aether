# Aether Foundation Implementation Plan

> **For agentic workers:** implement this plan task-by-task, in order, checking off each step as it lands. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build Aether's first honest end-to-end vertical slice: fetch an HTTPS document, parse a small HTML subset into a DOM, lay out block text, paint it into a native desktop window, and test each layer.

**Architecture:** A small Rust workspace separates engine concerns from browser orchestration. The first renderer uses a CPU raster surface and a native window; this is intentionally a stepping stone toward the planned display-list/GPU compositor, not a substitute for it. Networking uses mature HTTP/TLS libraries, while HTML tokenization, DOM construction, layout, and painting belong to Aether.

**Tech Stack:** Rust 2021, Cargo workspace, `ureq` for HTTPS fetch, `winit` for the native window, `softbuffer` for presenting a CPU-rendered framebuffer, and `fontdue` for text rasterization.

---

### Task 1: Repository and architecture baseline

**Files:**
- Create: `Cargo.toml`
- Create: `README.md`
- Create: `ARCHITECTURE.md`
- Create: `ROADMAP.md`
- Create: `CONTRIBUTING.md`
- Create: `SECURITY.md`
- Create: `THIRD_PARTY_LICENSES.md`
- Create: `docs/adr/0001-core-architecture.md`
- Create: `.gitignore`

- [ ] **Step 1: Create the workspace metadata and project documentation.**
- [ ] **Step 2: Initialize Git and verify `cargo metadata` succeeds.**
- [ ] **Step 3: Commit the baseline.**

### Task 2: HTML tokenizer, DOM, and parser

**Files:**
- Create: `engine/html/src/lib.rs`
- Create: `engine/dom/src/lib.rs`
- Create: `tests/engine_smoke.rs`

- [ ] **Step 1: Add tests for headings, paragraphs, nested elements, attributes, and malformed closing tags.**
- [ ] **Step 2: Implement a small tokenizer and tree builder with explicit ownership.**
- [ ] **Step 3: Run `cargo test -p aether-html -p aether-dom`.**

### Task 3: Layout and display list

**Files:**
- Create: `engine/layout/src/lib.rs`
- Create: `engine/paint/src/lib.rs`

- [ ] **Step 1: Add tests for block stacking and text wrapping.**
- [ ] **Step 2: Implement block layout into a display list.**
- [ ] **Step 3: Run layout and paint tests.**

### Task 4: HTTPS navigation and native presentation

**Files:**
- Create: `browser/src/main.rs`
- Create: `browser/src/navigation.rs`
- Create: `browser/src/surface.rs`

- [ ] **Step 1: Add URL parsing and HTTPS fetch tests with a local fixture seam.**
- [ ] **Step 2: Implement the navigation pipeline and native window.**
- [ ] **Step 3: Run `cargo test --workspace` and `cargo run -p aether-browser -- https://example.com`.**

### Task 5: Evidence and next-stage roadmap update

**Files:**
- Modify: `README.md`
- Modify: `ROADMAP.md`
- Create: `docs/evidence/foundation.md`

- [ ] **Step 1: Record exact test/build commands and known limitations.**
- [ ] **Step 2: Verify no claim exceeds the evidence.**
- [ ] **Step 3: Commit the vertical slice.**

## Spec coverage and explicit gaps

This plan covers only the first action and earliest milestones in the brief. It does not claim modern CSS, JavaScript, web APIs, process isolation, media, extensions, DevTools, PWA installation, or the 300+ application compatibility catalogue. Those require separate dependency-aware plans after this foundation is observable and testable.
