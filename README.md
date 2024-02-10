## Setup

1. Install Rust Tools: https://www.rust-lang.org/tools/install
2. Install tool for building wasm: ``` bash rustup target add wasm32-unknown-unknown```
3. Install cosmwasm-check: ```bash rustup target add wasm32-unknown-unknown```

## Create and Test

```bash
cargo t #unit tests
cargo build --target wasm32-unknown-unknown --release #to build wasm binary
cosmwasm-check ./target/wasm32-unknown-unknown/release/smart_contract.wasm
```

