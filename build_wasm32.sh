if [ ! -d target ]; then
mkdir target
fi
if [ ! -d target/deploy ]; then
mkdir target/deploy
fi

source "$HOME/emsdk/emsdk_env.sh" # Replace with your emscripten sdk's path
# export EMCC_CFLAGS="-s USE_SDL=2 -o web/index.html"
cargo build --target=wasm32-unknown-emscripten
python3 -m http.server --directory target/deploy
