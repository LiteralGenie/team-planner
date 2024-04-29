cargo install wasm-bindgen-cli wasm-pack
rustup target install wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown
wasm-pack build \
    --target web \
    --out-dir "../gui/src/lib/assets/wasm" \
