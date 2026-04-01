# Scrap Gamblers

A Fallout Pip-Boy styled interactive menu rendered in WebGL via Bevy + WASM. Features a CRT screen with fish-eye/bulge distortion, scanline post-processing, and physical-looking buttons with click sounds and tactile visual feedback.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)

## Running Natively

The fastest way to iterate during development:

```bash
cargo run
```

For faster incremental rebuilds and the Bevy Remote inspector:

```bash
cargo run --features dev
```

> Note: the `dev` feature enables `bevy/dynamic_linking` and `bevy/bevy_remote` — native only, do not use for WASM builds.

## Running in the Browser (WASM)

### One-time setup

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Dev server with hot reloading

```bash
trunk serve
```

Open [http://localhost:8080](http://localhost:8080) in your browser.

### Manual WASM build

```bash
cargo install wasm-bindgen-cli  # one-time

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir out --target web target/wasm32-unknown-unknown/release/scrap-gamblers.wasm
```

Then serve the `out/` directory with any static file server.

## Other Commands

```bash
cargo check   # type-check without building
cargo test    # run tests
```
