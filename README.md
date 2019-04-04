# Rust WASI Example

A toy program implementing [HQ9+][3] to demo compiling Rust to [WASI][1] and running it with [Wasmer][2].

## Setting up

Ensure you have an up to date version of Rust nightly and run:

```
rustup target add wasm32-unknown-wasi --toolchain nightly
```

## Building

```
cargo +nightly build --target=wasm32-unknown-wasi
cp target/wasm32-unknown-wasi/debug/wasi-example.wasm .
```

## Running

```
cd ../wasmer # go to Wasmer source directory
cargo run --release --features="wasi" -- run ../rust-wasi-example/wasi-example.wasm -- -e "HQ9+"
```

[1]: https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/
[2]: https://github.com/wasmerio/wasmer
[3]: https://esolangs.org/wiki/HQ9%2B