TARGET=wasm32-unknown-unknown
OUT_DIR=./target/wasm32-unknown-unknown/out
FONTS_DIR=./assets/fonts

cargo build --release --target $TARGET
wasm-bindgen --out-dir $OUT_DIR/ --target web target/$TARGET/release/pong_bevy.wasm

cp ./export/wasm/index.html $OUT_DIR/
cp ./export/wasm/pixel.ttf $OUT_DIR/
cp ./export/wasm/favicon.ico $OUT_DIR/

mkdir -p $OUT_DIR/$FONTS_DIR
cp $FONTS_DIR/Volter__28Goldfish_29.ttf $OUT_DIR/$FONTS_DIR
