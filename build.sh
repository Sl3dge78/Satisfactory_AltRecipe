#!/bin/sh
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/satisfactory_alt_recipe.wasm out/satisfactory_alt_recipe.wasm
basic-http-server out/
