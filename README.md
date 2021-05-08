# Parallel Raytracing on the Web in Rust
Based off of the books by Peter Shirley: https://raytracing.github.io/

This is my second time going through these tutorials.  My [https://github.com/evan-brass/weekend-raytracing](last version) got to about section 9 or 10.  I really enjoyed working on it, however there a few reasons I want to start fresh:
* I want to use SharedArrayBuffer and more than one worker thread.
	* I intend to setup COOP and COEP on something like Vercel if I can't get github pages working
* I've mostly forgotten how it worked

## 0. Initial Setup
* Setup Rust for threaded wasm

---
## Commands
* build: `cargo build; wasm-opt -Oz -o wasm/webtrace.wasm target/wasm32-unknown-unknown/debug/webtrace.wasm`
* build release: `cargo build --release; wasm-opt -Oz -o wasm/webtrace.wasm target/wasm32-unknown-unknown/release/webtrace.wasm`
* test: `cargo tt` (Make sure that the `tt` alias is setup with your machine's target-triple)

## Useful docs
* Cargo config.toml: https://doc.rust-lang.org/cargo/reference/config.html

## Tools
* [https://github.com/WebAssembly/binaryen](binaryen): wasm-opt

## Resources
* https://www.youtube.com/watch?v=gBPNO6ruevk&list=PL5B692fm6--sgm8Uiava0IIvUojjFOCSR

## Troubleshooting
* https://github.com/rustwasm/wasm-bindgen/issues/2102
* https://github.com/rust-lang/stdarch/pull/1073
* https://lld.llvm.org/WebAssembly.html
* https://github.com/rust-lang/rust/issues/77839
* https://github.com/bytecodealliance/wasmtime/issues/1658
* https://doc.rust-lang.org/nightly/src/core/up/up/stdarch/crates/core_arch/src/wasm32/atomic.rs.html