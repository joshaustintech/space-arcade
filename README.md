# space-arcade
Vertical space shooter inspired by @ianthetechie's game Bit Blaster (no longer available)

# usage
## prerequisites
- Rust 1.80.1 or newer
- Python 3
- Raylib prerequisites (CMake, etc.)
- Emscripten
- Add wasm32 emscripten compile target
```sh
$ rustup target add wasm32-unknown-emscripten
```
## compile/run instructions for desktop
```sh
$ cargo run
```
## compile instructions for web
```sh
$ make debug
```