Application embedding Wasmtime with a host function that reads WASM memory.

```
# 1. (optional) Build helloworld.wasm
emcc helloworld.c -O3 -o helloworld.wasm -s ERROR_ON_UNDEFINED_SYMBOLS=0
# 2. Build embedder app
cargo build
# 3. Run
target/debug/wasmtime-bad-example helloworld.wasm
```
