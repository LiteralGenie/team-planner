cargo install wasm-bindgen-cli wasm-server-runner
rustup target install wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown
wasm-bindgen \
    --target web \
    --out-dir "../gui/src/lib/assets/wasm" \
    --out-name "tft-core" \
    ./target/wasm32-unknown-unknown/debug/tft-core.wasm

