echo "Building wasm..."

wasm-bindgen target/wasm32-unknown-unknown/debug/image_black_white.wasm \
  --out-dir .

echo "Done."
