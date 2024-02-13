# A simple implementation of the conway's game of life written in rust.

## To Build and Run this project you need to have the following installed:

- Rust
- Cargo
- SDL2 (refer to the [guide of the embedded-graphics crate](https://docs.rs/embedded-graphics-simulator/0.6.0/embedded_graphics_simulator/#setup) for installation)


## The game is implemented using embedded-graphics and runs on a 64x64 monochrome display.
![image](img/conways-Game-Of-Life.png)

### Keybinds:
- `Space` to pause/unpause the game
- `TAB` to reset the game
- `Arrow keys` to move the cursor
- `Enter` to toggle the state of the cell under the cursor when the game is paused
- `Escape` to exit the game
- `LMB` to spawn a spaceship at the position of the cursor

Have fun!