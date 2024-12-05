#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CellState {
    Dead,
    Alive,
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<CellState>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![CellState::Dead; width]; height],
            width,
            height,
        }
    }

    pub fn set_dead(&mut self, x: usize, y: usize) {
        self.grid[y][x] = CellState::Dead;
    }
    pub fn set_alive(&mut self, x: usize, y: usize) {
        self.grid[y][x] = CellState::Alive;
    }

    pub fn get(&self, x: usize, y: usize) -> CellState {
        self.grid[y][x].clone()
    }

    /// Calculate the next generation of cells based on the rules of Conway's Game of life
    /// https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
    pub fn next_cell_generation(&mut self) {
        let mut new_grid = vec![vec![CellState::Dead; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                let current_state = &self.grid[y][x];

                new_grid[y][x] = match (current_state, neighbors) {
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
        grid.set_alive(0, 0); // Set top-left cell alive
        assert_eq!(grid.count_neighbors(1, 1), 1);
    }

    #[test]
    fn test_count_neighbors_multiple_neighbors() {
        let mut grid = Grid::new(3, 3);
        // set_alive up a pattern around center cell
        grid.set_alive(0, 0); // Top-left
        grid.set_alive(1, 0); // Top
        grid.set_alive(2, 0); // Top-right
        grid.set_alive(0, 1); // Left
        grid.set_alive(2, 1); // Right
        grid.set_alive(0, 2); // Bottom-left
        grid.set_alive(1, 2); // Bottom
        grid.set_alive(2, 2); // Bottom-right

        // Center cell should have 8 neighbors
        assert_eq!(grid.count_neighbors(1, 1), 8);
    }

    #[test]
    fn test_count_neighbors_corner_case() {
        let mut grid = Grid::new(3, 3);
        grid.set_alive(0, 1); // Set middle-left cell alive
        grid.set_alive(1, 0); // Set top-middle cell alive

        // Top-left corner should have 2 neighbors
        assert_eq!(grid.count_neighbors(0, 0), 2);
    }

    #[test]
    fn test_count_neighbors_edge_case() {
        let mut grid = Grid::new(3, 3);
        grid.set_alive(0, 0); // Top-left
        grid.set_alive(1, 0); // Top-middle
        grid.set_alive(2, 0); // Top-right
        grid.set_alive(2, 1); // Middle-right

        // Middle-top cell should have 3 neighbors
        assert_eq!(grid.count_neighbors(1, 0), 3);
    }

    // Rule 1: Any live cell with fewer than two live neighbours dies (referred to as underpopulation).
    #[test]
    fn test_rule_1() {
        let mut grid = Grid::new(3, 3);
        grid.set_alive(1, 1); // Set center cell Alive
        grid.set_alive(1, 2); // Set top-left cell Alive

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
        grid.set_alive(1, 1); // Set center cell Alive
        grid.set_alive(0, 0); // Set top-left cell Alive
        grid.set_alive(0, 1); // Set top-middle cell Alive

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
        grid.set_alive(0, 0); // Set top-left cell Alive
        grid.set_alive(0, 1); // Set top-middle cell Alive
        grid.set_alive(1, 0); // Set middle-left cell Alive

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
        grid.set_alive(1, 1); // Set center cell Alive
        grid.set_alive(0, 0); // Set top-left cell Alive
        grid.set_alive(0, 1); // Set top-middle cell Alive
        grid.set_alive(0, 2); // Set top-right cell Alive
        grid.set_alive(1, 0); // Set middle-left cell Alive

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

        grid.set_alive(2, 1);
        grid.set_alive(2, 2);
        grid.set_alive(2, 3);

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
        grid.set_alive(2, 2);
        grid.set_alive(3, 2);
        grid.set_alive(4, 2);
        grid.set_alive(1, 3);
        grid.set_alive(2, 3);
        grid.set_alive(3, 3);

        println!("Initial generation:");
        print_grid(&grid);

        grid.next_cell_generation();
        println!("Next generation:");
        print_grid(&grid);
    }
}
