#!/usr/bin/env bash

# Many thanks to https://github.com/tung/ruggrogue/blob/master/book/dependencies.md

if [ ! -d target/deploy ]; then
mkdir -p target/deploy
fi

source "$HOME/emsdk/emsdk_env.sh" && # Replace with your emscripten sdk's path
cargo build --bin rui_research --target=wasm32-unknown-emscripten --release &&
echo &&
echo You can now open a browser at http://localhost:8000 &&
python3 -m http.server --directory target/deploy
