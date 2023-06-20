# How to run it

## Linux

### Ubuntu:
1. Install Rust
   1. With rustup: https://www.rust-lang.org/tools/install
   2. Or distro packages
      > sudo apt install rust-all
2. Install SDL2 dev libraries
   >  sudo apt install libsdl2-dev
3. Run it
   > cargo run

### Alpine linux
1. Install Rust
   > sudo apk add cargo
2. Install SDL2 dev libraries
   > sudo apk add sdl2-dev
3. Run it
   > cargo run

## Windows:
1. Install Rust: https://www.rust-lang.org/tools/install 
2. Install SDL2: https://wiki.libsdl.org/SDL2/Installation
3. Run it
   > cargo run

## Web:
1. Install Rust: https://www.rust-lang.org/tools/install
2. Install target
   > rustup target add wasm32-unknown-emscripten
3. Install emscripten: https://emscripten.org/docs/getting_started/downloads.html
4. Open build_wasm32.sh and update EMSDK_PATH to point it to your emsdk installation's path
5. Run it
   > ./build_wasm32.sh
6. Open a browser at http://localhost:8000
 
## Performance:
1. Install framegraph
   > cargo install flamegraph
2. Install framegraph's Linux requirements
   > sudo apt install linux-tools-common linux-tools-generic linux-tools-`uname -r`
3. Run it
   > ./run_profiler.sh