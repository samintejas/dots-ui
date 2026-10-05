#!/usr/bin/env bash
# Builds the gallery into gallery/dist (needs wasm-bindgen-cli matching the wasm-bindgen in Cargo.lock).
set -euo pipefail
cd "$(dirname "$0")/.."
cargo build --target wasm32-unknown-unknown -p dots-gallery
rm -rf gallery/dist && mkdir -p gallery/dist/fonts
wasm-bindgen --target web --no-typescript --out-dir gallery/dist/pkg target/wasm32-unknown-unknown/debug/dots-gallery.wasm
cp gallery/index.html gallery/dist/index.html
cp assets/fonts/*.woff2 gallery/dist/fonts/
echo "built gallery/dist; serve it with: python3 -m http.server -d gallery/dist 8080"
