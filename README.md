# Conway's game of life
Yet another Conway's game of life implementation but in Rust.

## Usage

- Clone the repository and navigate to the project directory.
  ```shell
      git clone https://github.com/LucasUTNFRD/Conways.git && cd Conways
  ```
- Run the project
  ```shell
      make run
  ```

## Functionality
Conway's Game of Life is a cellular automaton simulation where each cell can be either alive or dead based on the following rules:

- Any live cell with fewer than two live neighbours dies, as if by underpopulation
- Any live cell with two or three live neighbours lives on to the next generation
- Any live cell with more than three live neighbours dies, as if by overpopulation
- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction

The game runs automatically, showing how patterns evolve over successive generations. You can:

- See different patterns emerge and evolve
- Watch how stable structures form
- Observe oscillating and gliding patterns
- Click any cell to toggle its state between alive/dead
- Pause/resume the simulation with spacebar

## TODO
- [ ] Add gif of the game running

