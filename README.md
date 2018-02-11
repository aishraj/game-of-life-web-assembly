## Game of Life ##

This is an experimental implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in Rust. 

![Screenshot](https://raw.githubusercontent.com/aishraj/game-of-life-web-assembly/master/screenshots/glider-gen.png)


### Running it on your computer ###

First, make sure you have a Rust compiler that supports the `wasm32-unknown-unknown` target.
You can set it up by following the instructions [here](https://www.hellorust.com/setup/wasm-target/)

Next, clone this repostiory and in the checked out directory root, run the following to compile the code

`cargo +nightly build --target wasm32-unknown-unknown --release`

Next, copy the generated `wasm` file to the `wasm` directory.

`cp target/wasm32-unknown-unknown/release/game_of_life_web_assembly.wasm ./web/wasm/life.wasm`

After this, `cd` into `web` and start an HTTP server. Your page should now have beautiful WebAssembly code running.

#### Why yet another Game of Life implementation ? ####
Beacuse, I feel that Game of Life is one of the simplest way to get started with a new tool that involves graphics/animation. It does not require any user input and is quite easy to implement. I wrote this particular version mostly to experiment with Rust's `wasm32-unknown-unknown` target, as opposed to using `emsdk` and the `emscripten` toolchain.

## Credits ##

_“Imitation is the sincerest form of flattery" - Oscar Wilde_

 - Most of the code here was inspired by [Rocket Wasm](https://aochagavia.github.io/blog/rocket---a-rust-game-running-on-wasm/).
- The Canvas specific parts have been based on [JSFiddle](http://jsfiddle.net/ankr/tgjLA/) found in the wild.

