if [ ! -d target/deploy ]; then
mkdir -p target/deploy
fi

source "$HOME/emsdk/emsdk_env.sh" # Replace with your emscripten sdk's path
# export EMCC_CFLAGS="-s USE_SDL=2 -o web/index.html"
cargo build --target=wasm32-unknown-emscripten --release
echo
echo You can now open a browser at http://localhost:8000
python3 -m http.server --directory target/deploy
