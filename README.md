# space-arcade
Vertical space shooter inspired by @ianthetechie's game Bit Blaster (no longer available)

# usage
## prerequisites
- Rust 1.80.1 or newer
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
## macOS compile instructions for web
```sh
$ export EMCC_CFLAGS="-O3 -sUSE_GLFW=3 -sGL_ENABLE_GET_PROC_ADDRESS -sWASM=1 -sALLOW_MEMORY_GROWTH=1 -sWASM_MEM_MAX=512MB -sTOTAL_MEMORY=512MB -sABORTING_MALLOC=0 -sASYNCIFY -sFORCE_FILESYSTEM=1 -sASSERTIONS=1 -sERROR_ON_UNDEFINED_SYMBOLS=0 -sEXPORTED_RUNTIME_METHODS=ccallcwrap"
$ cargo build --target wasm32-unknown-emscripten
```