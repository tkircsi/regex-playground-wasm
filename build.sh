#!/bin/bash

# Create frontend/wasm directory if it doesn't exist
mkdir -p frontend/wasm

# Build the WASM module
cd regex_engine
wasm-pack build --target web --out-dir ../frontend/wasm
