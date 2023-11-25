#!/usr/bin/env bash
# Many thanks to https://github.com/tung/ruggrogue/blob/master/book/dependencies.md
# And https://web.archive.org/web/20210618192808/https://blog.therocode.net/2020/10/a-guide-to-rust-sdl2-emscripten

# Update this to point it to your emsdk installation's path
EMSDK_PATH=$HOME/emsdk

if [ ! -d target/deploy ]; then
  mkdir -p target/deploy
fi

source "$EMSDK_PATH/emsdk_env.sh" &&
  cargo build --bin widget_gallery --target=wasm32-unknown-emscripten --release &&
  echo &&
  echo &&
  echo =========================================================================== &&
  echo You can open a browser at http://localhost:8000. CTRL-C to close the server &&
  python3 -m http.server --directory target/deploy
