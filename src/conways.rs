#[derive(Clone, PartialEq, Eq, Debug)]
/// Represents the state of a cell in Conway's Game of Life
/// - `Dead`: An inactive/empty cell
/// - `Alive`: An active/populated cell
pub enum CellState {
    Dead,
    Alive,
}

/// Represents a 2D grid of cells
/// The grid is represented as a vector of vectors of `CellState`
/// Each cell can be in one of two states: Dead or Alive
pub struct Grid {
    grid: Vec<Vec<CellState>>,
    width: usize,
    height: usize,
}

impl Grid {
    /// Create a new grid with the specified width and height
    /// All cells are initialized to `Dead`
    ///
    /// # Arguments
    /// * `width` - The width of the grid
    /// * `height` - The height of the grid
    ///
    /// # Returns
    /// A new `Grid` instance with all cells initialized to `Dead`
    ///
    /// # Example
    /// ```
    /// let grid = Grid::new(10, 10);
    /// ```
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![CellState::Dead; width]; height],
            width,
            height,
        }
    }

    /// Set the state of a cell at the specified coordinates
    ///
    /// # Arguments
    /// * `x` - The x-coordinate of the cell
    /// * `y` - The y-coordinate of the cell
    /// * `state` - The state to set the cell to
    ///
    /// # Example
    /// ```
    /// let mut grid = Grid::new(10, 10);
    /// grid.set(5, 5, CellState::Alive);
    /// ```
    pub fn set(&mut self, x: usize, y: usize, state: CellState) {
        self.grid[y][x] = state;
    }

    /// Gets the current state of a cell at the specified coordinates.
    ///
    /// # Arguments
    /// * `x` - The x-coordinate (column) of the cell
    /// * `y` - The y-coordinate (row) of the cell
    ///
    /// # Returns
    /// The current `CellState` of the cell at the specified position
    ///
    /// # Example
    /// ```
    /// let grid = Grid::new(10, 10);
    /// let cell_state = grid.get(5, 5);
    /// ```
    pub fn get(&self, x: usize, y: usize) -> CellState {
        self.grid[y][x].clone()
    }

    /// Advances the grid to the next generation according to Conway's Game of Life rules:
    /// 1. Any live cell with fewer than two live neighbors dies (underpopulation)
    /// 2. Any live cell with two or three live neighbors survives
    /// 3. Any live cell with more than three live neighbors dies (overpopulation)
    /// 4. Any dead cell with exactly three live neighbors becomes alive (reproduction)
    ///
    /// This method updates the entire grid based on these rules, creating the next generation
    /// of cells.
    /// see more: <https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life>
    pub fn next_cell_generation(&mut self) {
        let mut new_grid = vec![vec![CellState::Dead; self.width]; self.height];

        for (y, row) in new_grid.iter_mut().enumerate().take(self.height) {
            for (x, cell) in row.iter_mut().enumerate().take(self.width) {
                let neighbors = self.count_neighbors(x, y);
                let current_state = &self.grid[y][x];

                *cell = match (current_state, neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbors dies
                    (CellState::Alive, 0..=1) => CellState::Dead,
                    // Rule 2: Any live cell with two or three live neighbors lives
                    (CellState::Alive, 2..=3) => CellState::Alive,
                    // Rule 3: Any live cell with more than three live neighbors dies
                    (CellState::Alive, 4..=8) => CellState::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbors becomes alive
                    (CellState::Dead, 3) => CellState::Alive,
                    // All other cells remain in their current state
                    (state, _) => state.clone(),
                };
            }
        }

        self.grid = new_grid;
    }

    /// Count the number of alive neighbors for a given cells
    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                // Skip the center cell (the cell itself)
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if self
                    .grid
                    .get(ny as usize)
                    .and_then(|row| row.get(nx as usize))
                    .map_or(false, |cell| *cell == CellState::Alive)
                {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_neighbors_empty_grid() {
        let grid = Grid::new(3, 3);
        assert_eq!(grid.count_neighbors(1, 1), 0);
    }

    #[test]
    fn test_count_neighbors_single_neighbor() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, CellState::Alive); // Set top-left cell alive
        assert_eq!(grid.count_neighbors(1, 1), 1);
    }

    #[test]
    fn test_count_neighbors_multiple_neighbors() {
        let mut grid = Grid::new(3, 3);
        // set up a pattern around center cell
        grid.set(0, 0, CellState::Alive); // Top-left
        grid.set(1, 0, CellState::Alive); // Top
        grid.set(2, 0, CellState::Alive); // Top-right
        grid.set(0, 1, CellState::Alive); // Left
        grid.set(2, 1, CellState::Alive); // Right
        grid.set(0, 2, CellState::Alive); // Bottom-left
        grid.set(1, 2, CellState::Alive); // Bottom
        grid.set(2, 2, CellState::Alive); // Bottom-right

        // Center cell should have 8 neighbors
        assert_eq!(grid.count_neighbors(1, 1), 8);
    }

    #[test]
    fn test_count_neighbors_corner_case() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 1, CellState::Alive); // Set middle-left cell alive
        grid.set(1, 0, CellState::Alive); // Set top-middle cell alive

        // Top-left corner should have 2 neighbors
        assert_eq!(grid.count_neighbors(0, 0), 2);
    }

    #[test]
    fn test_count_neighbors_edge_case() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, CellState::Alive); // Top-left
        grid.set(1, 0, CellState::Alive); // Top-middle
        grid.set(2, 0, CellState::Alive); // Top-right
        grid.set(2, 1, CellState::Alive); // Middle-right

        // Middle-top cell should have 3 neighbors
        assert_eq!(grid.count_neighbors(1, 0), 3);
    }

    // Rule 1: Any live cell with fewer than two live neighbours dies (referred to as underpopulation).
    #[test]
    fn test_rule_1() {
        let mut grid = Grid::new(3, 3);
        grid.set(1, 1, CellState::Alive); // Set center cell Alive
        grid.set(1, 2, CellState::Alive); // Set top-left cell Alive

        //check that center cell only has 1 neighbor
        assert_eq!(grid.count_neighbors(1, 1), 1);

        //check that center cell dies after next generation
        grid.next_cell_generation();
        assert_eq!(grid.get(1, 1), CellState::Dead);
    }

    // Rule 2: Any live cell with two or three live neighbours lives, unchanged, to the next generation.
    #[test]
    fn test_rule_2() {
        let mut grid = Grid::new(3, 3);
        grid.set(1, 1, CellState::Alive); // Set center cell Alive
        grid.set(0, 0, CellState::Alive); // Set top-left cell Alive
        grid.set(0, 1, CellState::Alive); // Set top-middle cell Alive

        //check that center cell has 2 neighbors
        assert_eq!(grid.count_neighbors(1, 1), 2);

        //check that center cell lives after next generation
        grid.next_cell_generation();
        assert_eq!(grid.get(1, 1), CellState::Alive);
    }

    // Rule 3: Any dead cell with exactly three live neighbours comes to life.
    #[test]
    fn test_rule_3() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, CellState::Alive); // Set top-left cell Alive
        grid.set(0, 1, CellState::Alive); // Set top-middle cell Alive
        grid.set(1, 0, CellState::Alive); // Set middle-left cell Alive

        //check that center cell has 3 neighbors
        assert_eq!(grid.count_neighbors(1, 1), 3);

        //check that center cell comes to life after next generation
        println!("Before:");
        print_grid(&grid);

        grid.next_cell_generation();

        println!("After:");
        print_grid(&grid);

        assert_eq!(grid.get(1, 1), CellState::Alive);
    }

    // Rule 4: Any live cell with more than three live neighbours dies (referred to as overpopulation).
    #[test]
    fn test_rule_4() {
        let mut grid = Grid::new(3, 3);
        grid.set(1, 1, CellState::Alive); // Set center cell Alive
        grid.set(0, 0, CellState::Alive); // Set top-left cell Alive
        grid.set(0, 1, CellState::Alive); // Set top-middle cell Alive
        grid.set(0, 2, CellState::Alive); // Set top-right cell Alive
        grid.set(1, 0, CellState::Alive); // Set middle-left cell Alive

        //check that center cell has 4 neighbors
        assert_eq!(grid.count_neighbors(1, 1), 4);

        //check that center cell dies after next generation
        grid.next_cell_generation();
        assert_eq!(grid.get(1, 1), CellState::Dead);
    }

    fn print_grid(grid: &Grid) {
        for row in &grid.grid {
            for cell in row {
                match cell {
                    CellState::Dead => print!(". "),
                    CellState::Alive => print!("O "),
                }
            }
            println!();
        }
        println!();
    }

    #[test]
    fn test_blinker_pattern() {
        let mut grid = Grid::new(5, 5);

        grid.set(2, 1, CellState::Alive);
        grid.set(2, 2, CellState::Alive);
        grid.set(2, 3, CellState::Alive);

        println!("Initial generation:");
        print_grid(&grid);

        grid.next_cell_generation();
        println!("Next generation:");
        print_grid(&grid);
    }

    #[test]
    fn test_toad_pattern() {
        let mut grid = Grid::new(6, 6);

        // Set toad pattern
        grid.set(2, 2, CellState::Alive);
        grid.set(3, 2, CellState::Alive);
        grid.set(4, 2, CellState::Alive);
        grid.set(1, 3, CellState::Alive);
        grid.set(2, 3, CellState::Alive);
        grid.set(3, 3, CellState::Alive);

        println!("Initial generation:");
        print_grid(&grid);

        grid.next_cell_generation();
        println!("Next generation:");
        print_grid(&grid);
    }
}
