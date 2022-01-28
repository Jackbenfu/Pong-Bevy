cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir ./target/wasm32-unknown-unknown/out/ --target web target/wasm32-unknown-unknown/release/pong_bevy.wasm
cp ./export/wasm/index.html ./export/wasm/pixel.ttf ./export/wasm/favicon.ico ./target/wasm32-unknown-unknown/out/
