# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Goal

A Fallout Pip-Boy styled interactive menu rendered in WebGL (via Bevy + WASM). The UI features:
- A CRT screen in the center with fish-eye/bulge distortion and scanline post-processing
- Physical-looking buttons flanking the screen with click sounds and tactile visual feedback
- Text-based menu navigation driven by the buttons

## Commands

```bash
# Native desktop run (fast iteration)
cargo run

# Build for WebAssembly (browser)
cargo build --release --target wasm32-unknown-unknown

# Run tests
cargo test

# Check without building
cargo check
```

### WebAssembly Setup

The WASM target and `wasm-bindgen-cli` must be installed once:
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
```

After a WASM build, run `wasm-bindgen` to produce JS/HTML glue, then serve with any static file server. The `dynamic_linking` feature must be **disabled** for WASM builds — use a separate feature flag or override at build time.

`trunk` is an alternative that automates the wasm-bindgen + serve workflow:
```bash
cargo install trunk
trunk serve   # hot-reloading dev server at localhost:8080
```

## Architecture

Built on **Bevy 0.18** using its ECS pattern. Key subsystems to implement:

### Rendering Pipeline
- Use Bevy's `Material2d` or a custom `RenderPipeline` with a WGSL shader for the CRT effect (scanlines, barrel/fish-eye distortion, phosphor glow). Render the menu to a `RenderTarget` texture, then display that texture through the CRT shader on a fullscreen quad.

### UI / Menu System
- Menu state lives as a Bevy `Resource` (current tab, selected item, etc.).
- Text rendered via Bevy's `Text` components inside the CRT render target.
- Navigation driven by `ButtonPressed` events dispatched from the physical button entities.

### Input / Buttons
- Each on-screen button is a Bevy entity with a `Button` component and a custom `PipBoyButton` marker.
- Systems listen for `Interaction` state changes, fire navigation events, trigger a short animation (press-depth transform), and play a click `AudioSource`.

### Audio
- Click sound loaded as an `Asset<AudioSource>` and played via `AudioPlayer` on button press.

## Dev Profile

`Cargo.toml` compiles dependencies at `opt-level = 3` even in debug builds to keep Bevy render performance acceptable during iteration. The `dynamic_linking` feature speeds up incremental native builds but must be stripped for WASM targets.
