# starrock

Starrock is an Astroid clone that is a experiment in the possibility of combining Rust, Web-Assembly
and WebGL. And it works surprisingly well!

## Play

It is possible to play the game [here](https://felixnaredi.github.io/starrock/). It is not a finnished
product but it can be pretty fun to fly around (with WASD), hit rocks and shoot lasers with SPACE.

## Installation

The first time you run the program from the repository, run the following commands in the terminal to do some initial installations:
```
cargo install wasm-pack
yarn install
```

After the installation run:
```
yarn serve
```
Now all *Rust* dependencies will be installed. When that is done, open your browser and visit *http://localhost:8080/* to play the game!
