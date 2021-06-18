# Conway's Game of Life

Conway's game of life written in Rust using SDL.

# Requirements

* [the standard Rust toolchain](https://www.rust-lang.org/tools/install).

* an up to date C compiler

* [cmake](https://cmake.org)


Enter the conways_game_of_life_sdl directory and run

a randomly populated grid:
```
cargo run --release --bin random
```

an empty grid:
```
cargo run --release --bin empty
```

# Controls

* Left-click and drag to spawn live cells

* Right-click and drag to move the camera

* Mouse scroll or +/- keys to zoom in and out

* Space - pause

* Enter - advance one step while paused

* R - reset

* Escape - close the window
