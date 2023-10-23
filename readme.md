# Rusty Corks

## Gameplay

Come and do the impossible in Rusty Corks, where you have to drive a car in the opposite direction to the traffic!
Drive to the rhythm of the music to ensure your survival, but be careful: the more cars you overtake,
the faster they go!

![game_play](./assets/documentation/in_game.png)

## Features

- You can pause the game by pressing `space` (with default key bindings)
- You can change the key bindings in the `config.rs` file
- You can change the music in the `config.rs` file
- Show key bindings in the main menu
- Bots spawn randomly
- Speed increases as you progress
- Music is played in the background
- Score is displayed in the top left corner
- Game over screen is displayed when you lose 

## How to play
First, you need to clone the repository. \
Install [rustup](https://www.rust-lang.org/tools/install) from the official website and run the following command:

### If you want to build the game
```bash
cargo build --release
```

### If you want to run the game
```bash
cargo run --release
```