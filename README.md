# About this

## Vision
\<Insert Name Here\> The Little GUI is an rust-written open-source cross-platform multi-paradigm convergent GUI library that
tries to take back the fun and beauty of creating UI applications by moving the approach a
little more retro.

* It avoids using web technologies to render the UI, although it can render to many platforms: 
Web, Linux, Windows, Android.
* It avoids parsing strings to make it faster, also it renders the UI by building the polygons in parallel.
* It uses less memory by not having to load a browser to run the application.
* It is multi-paradigm by having both declarative and imperative approaches. Also inspired by web, it declares the style 
separate to the contents.
* It will (not yet) handle fractional-scaling automatically.
* It is small because it only handles GUI stuff and assets, for handling other stuff please check other libraries e.g. 
SDL.

Currently, It is implemented with SDL but in the future it might have native web and TUI implementations.

## Motivation
Many years ago, my friend and I were developing a NES emulator, and we were never able to create a GUI for it, mostly 
because there was no open-source cross-platform library that will not take ownership of the main thread and will work 
easily with SDL. There are workarounds around this, but we never dedicated too much time to make them work.

Also, as part of learning the beautiful rust language, I challenged myself to put such knowledge in practice and here we are.  


# System dependencies
1. Rust
2. SDL >2.0.18


# How to run it

## Linux

### Ubuntu
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

## Windows
1. Install Rust: https://www.rust-lang.org/tools/install 
2. Install SDL2: https://wiki.libsdl.org/SDL2/Installation
3. Run it
   > cargo run

## Web
1. Install Rust: https://www.rust-lang.org/tools/install
2. Install target
   > rustup target add wasm32-unknown-emscripten
3. Install emscripten: https://emscripten.org/docs/getting_started/downloads.html
4. Open build_wasm32.sh and update EMSDK_PATH to point it to your emsdk installation's path
5. Run it
   > ./build_wasm32.sh
6. Open a browser at http://localhost:8000

## Android
1. Setup Android environment
2. Setup configuration section in the build_android.py file
3. Run it
   > ./build_android.py
 
## Performance
1. Install framegraph
   > cargo install flamegraph
2. Install framegraph's Linux requirements
   > sudo apt install linux-tools-common linux-tools-generic linux-tools-\`uname -r`
3. Run it
   > ./run_profiler.sh


# How to debug with vscodium
1. Install rustup
2. Install vscodium
3. Install rust-analyzer extension
4. Install CodeLLDB extension