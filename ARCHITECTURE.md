# Aether Architecture

Aether is organized as an owned browser pipeline: navigation fetches bytes, HTML builds a DOM, layout produces positioned text runs, and paint produces a framebuffer for the native surface. Each stage has a small Rust crate so tests can exercise it without opening a window.

The current framebuffer is deliberately a temporary CPU renderer. The next rendering boundary is a display list, followed by clipping, images, text shaping, and a GPU compositor. Mature dependencies are restricted to isolated platform/network primitives; no embedded browser engine is used.

