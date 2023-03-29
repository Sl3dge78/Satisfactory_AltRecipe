#!/bin/sh
cargo build --release --target wasm32-unknown-unknown
#wasm-bindgen --out-name satisfactory_alt_recipe --out-dir out --target web target/wasm32-unknown-unknown/release/satisfactory_alt_recipe.wasm
cp target/wasm32-unknown-unknown/release/satisfactory_alt_recipe.wasm out/satisfactory_alt_recipe.wasm
basic-http-server out/
