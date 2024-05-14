cargo install wasm-bindgen-cli wasm-pack
rustup target install wasm32-unknown-unknown

cargo build --target wasm32-unknown-unknown --release
wasm-pack build \
    --target no-modules \
    --out-dir "../gui/static/wasm" \

cp ../gui/static/wasm/tft_core.d.ts ../gui/src/lib/tft_core.d.ts