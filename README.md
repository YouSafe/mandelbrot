# Mandel

![Screenshot](Screenshot_20260910_060406.png)

Live-Demo: https://yousafe.github.io/mandelbrot/

Support for WebGPU (default if supported), WebGL (fallback), Vulkan (Desktop)

## Controls

- Left Click Drag: Pan Camera
- Scroll: Zoom Camera
- Right Click Drag in Mandelbrot Set: Select $J_c$ for the Julia-Set

## Algorithms

- Escape time algorithm: Terminate when repeated iterations of $z_{n+1} = {z_n}^2 + c$ exceed bailout value (App uses value 8 for smoother gradients). Visualize number of iterations before "escape" condition is met.

- Fixed cyclic color palette: 16 Color Palette from [Stack Overflow](https://stackoverflow.com/a/16505538).

- Implemented on the GPU using a fragment shader.

## Build & Run Native

1. Install [Rust lang](https://rust-lang.org/tools/install/).
2. (Install possibly required system dependencies).
3. Run `cargo run --release`.

## Build & Run Web Version

1. Install [Rust lang](https://rust-lang.org/tools/install/).
2. Install the required target `rustup target add wasm32-unknown-unknown`.
3. Install Trunk `cargo install --locked trunk`.
4. Run `trunk serve` to build and open `http://127.0.0.1:8080` in a browser.

## Software used

- eframe_template: project template for using egui.
- wgpu: cross-platform graphics API.
- egui: immediate mode GUI in Rust.
- trunk: build wasm for web version.
