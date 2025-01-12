# Snake Game in Rust

This is a simple Snake game written in Rust using the `piston_window` crate for rendering. The game features a snake that moves around a board, eats food, and grows longer. The game ends when the snake collides with the border or itself. You can control the snake using the arrow keys.

## Features
- **Snake Movement**: The snake moves in four directions (Up, Down, Left, Right).
- **Food**: The snake eats food to grow longer. Food spawns randomly within the game board.
- **Game Over**: The game ends when the snake collides with the border or itself.
- **Restart**: After the game ends, the game can be restarted after a short wait time.

## Dependencies
- Rust 1.0 or newer
- `piston_window` for rendering
- `rand` for random number generation

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/snake_game.git
   cd snake_game
2. Ensure you have Rust installed. If you don't, follow the installation instructions from the official Rust website.
3. Add the required dependencies to Cargo.toml:
   ```bash
   [dependencies]
   piston_window = "0.120.0"
   rand = "0.8"
5. Build and run the game using Cargo


## Code Explanation

### `draw.rs`
Contains functions for rendering the game on the screen, including `to_coord`, `draw_block`, and `draw_rectangle`.

- **`to_coord(game_coord: i32) -> f64`**: Converts a game coordinate to a graphical coordinate by multiplying it with the block size.
- **`to_coord_u32(game_coord: i32) -> u32`**: Converts a game coordinate to a `u32` for rendering purposes.
- **`draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d)`**: Draws a single block on the screen.
- **`draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d)`**: Draws a rectangle made of blocks.

### `snake.rs`
Defines the `Snake` struct and its methods for managing the snake's state and movement.

- **`Direction`**: Enum for controlling the direction of the snake (Up, Down, Left, Right).
- **`Snake`**: Struct that represents the snake with methods for movement, growth, and drawing.
  - **`new(x: i32, y: i32)`**: Creates a new snake at the given coordinates.
  - **`draw(con: &Context, g: &mut G2d)`**: Draws the snake on the screen.
  - **`move_forward(dir: Option<Direction>)`**: Moves the snake in the given direction.
  - **`check_if_snake_alive(dir: Option<Direction>) -> bool`**: Checks if the snake is alive (not colliding with itself or the borders).

### `game.rs`
Contains the main game logic, including food spawning, snake updates, and game-over conditions.

- **`Game`**: Struct that manages the game state (snake, food, borders, etc.).
  - **`new(width: i32, height: i32)`**: Creates a new game with the specified dimensions.
  - **`key_pressed(key: Key)`**: Handles key presses for controlling the snake.
  - **`update(delta_time: f64)`**: Updates the game state, including snake movement and food generation.
  - **`check_eating()`**: Checks if the snake has eaten food.
  - **`add_food()`**: Generates new food in a random position.
  - **`restart()`**: Resets the game state.

### `main.rs`
The entry point of the game, initializing the window, handling events, and updating the game state.

- Creates a Piston window with the specified width and height.
- Listens for keyboard input and updates the game based on user input.
- Renders the game each frame.

## Controls
- **Arrow Keys**: Control the direction of the snake.
- **Escape**: Exit the game.

## Game Over
When the snake hits the border or itself, the game ends, and a "Game Over" screen will appear. The game can be restarted after a short wait time.
