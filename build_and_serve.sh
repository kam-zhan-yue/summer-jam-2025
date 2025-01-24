#!/bin/bash

# Add the wasm target for Rust
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli
cargo install -f wasm-bindgen-cli

# Build the project for the wasm target
cargo build --release --target wasm32-unknown-unknown

# Run wasm-bindgen to generate the necessary JS bindings
wasm-bindgen --out-dir ./webbuild/out/ --target web ./target/wasm32-unknown-unknown/release/summer_jam.wasm

# Copy the assets to the webbuild directory
cp -r assets ./webbuild/

# Extract the package name from Cargo.toml
PKG_NAME=$(grep '^name = ' Cargo.toml | awk -F '"' '{print $2}' | head -n 1)

# Replace the placeholder in index.html with the actual package name
sed -i '' "s/\/out\/summer_jam.js/\/out\/${PKG_NAME}.js/g" webbuild/index.html

# Serve the webbuild directory
npx serve webbuild

# Zip the file for Itch
zip -r webbuild/build.zip webbuild
