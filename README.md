# Rust Extism

```bash
rustup target add wasm32-wasip1

curl https://wasmtime.dev/install.sh -sSf | bash

cargo new wasi-hello-world


cargo build --target wasm32-wasip1

```


```bash
cargo new --lib my-plugin
cd my-plugin 
cargo add extism-pdk

```

```toml
[lib]
crate-type = ["cdylib"]
```

cargo build --target wasm32-unknown-unknown

cargo build --target wasm32-wasip1
