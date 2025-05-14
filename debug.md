

rustc 1.86.0 (05f9846f8 2025-03-31)
cargo 1.86.0 (adf9b6ad1 2025-02-28)

rustup target list --installed:
aarch64-unknown-linux-gnu
wasm32-unknown-unknown
wasm32-wasip1

cargo new --lib my-plugin
cd  my-plugin/
cargo add extism-pdk


```
cargo build --target wasm32-unknown-unknown

error[E0463]: can't find crate for `paste`
  --> /home/k33g/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rmp-0.8.14/src/decode/mod.rs:60:20
   |
60 |                 Ok(paste::paste! {
   |                    ^^^^^ can't find crate

error[E0463]: can't find crate for `paste`
   --> /home/k33g/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rmp-0.8.14/src/decode/mod.rs:146:11
    |
146 |         $(paste::paste! {
    |           ^^^^^ can't find crate

error[E0463]: can't find crate for `paste`
   --> /home/k33g/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rmp-0.8.14/src/encode/mod.rs:124:17
    |
124 |                 paste::paste! {
    |                 ^^^^^ can't find crate

For more information about this error, try `rustc --explain E0463`.
error: could not compile `rmp` (lib) due to 3 previous errors
warning: build failed, waiting for other jobs to finish...

```