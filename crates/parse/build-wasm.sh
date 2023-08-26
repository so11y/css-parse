#!/bin/bash


# chmod +x build-wasm.sh

wasm-pack build --target web --release


# cargo watch -s ./build-wasm.sh