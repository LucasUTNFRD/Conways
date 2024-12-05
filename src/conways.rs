use std::usize;

#[derive(Clone, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

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

    pub fn set(&mut self, x: usize, y: usize) {
        self.grid[y][x] = CellState::Alive;
    }

    pub fn get(&self, x: usize, y: usize) -> CellState {
        self.grid[y][x].clone()
    }

    ///
    /// Create a new grid
    /// Calculate the the new state for each cell based in the current generation
    /// Replace new grid with the current grid
    ///
    pub fn next_cell_generation(&mut self) {
        let mut new_grid = vec![vec![CellState::Dead; self.width]; self.height];

        for i in 0..self.height {
            for j in 0..self.width {
                let neighbour_count = self.count_neighbors(i, j);
                let current_state = &self.grid[i][j];

                //apply conwayws rule
                new_grid[i][j] = match (current_state, neighbour_count) {
                    // Rule 1
                    // Any live cell with fewer than two live neighbours dies (referred to as underpopulation).
                    (CellState::Alive, 0..=1) => CellState::Dead,
                    //Rule 2
                    //Any live cell with two or three live neighbours lives, unchanged, to the next generation.
                    (CellState::Alive, 2..=3) => CellState::Alive,
                    //Rule 3
                    //Any dead cell with exactly three live neighbours comes to life.
                    (CellState::Dead, 3) => CellState::Alive,
                    //Rule 4
                    //Any live cell with more than three live neighbours dies (referred to as overpopulation).
                    (CellState::Alive, 4..=8) => CellState::Dead,
                    // Otherwise
                    (state, _) => state.clone(),
                };
            }
        }
    }

    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                // Skip the center cell (the cell itself)
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                // Only count if within grid boundaries
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    if self.grid[ny as usize][nx as usize] == CellState::Alive {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

//Create test for conways logic
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
        grid.set(0, 0); // Set top-left cell alive
        assert_eq!(grid.count_neighbors(1, 1), 1);
    }

    #[test]
    fn test_count_neighbors_multiple_neighbors() {
        let mut grid = Grid::new(3, 3);
        // Set up a pattern around center cell
        grid.set(0, 0); // Top-left
        grid.set(1, 0); // Top
        grid.set(2, 0); // Top-right
        grid.set(0, 1); // Left
        grid.set(2, 1); // Right
        grid.set(0, 2); // Bottom-left
        grid.set(1, 2); // Bottom
        grid.set(2, 2); // Bottom-right

        // Center cell should have 8 neighbors
        assert_eq!(grid.count_neighbors(1, 1), 8);
    }

    #[test]
    fn test_count_neighbors_corner_case() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 1); // Set middle-left cell alive
        grid.set(1, 0); // Set top-middle cell alive

        // Top-left corner should have 2 neighbors
        assert_eq!(grid.count_neighbors(0, 0), 2);
    }

    #[test]
    fn test_count_neighbors_edge_case() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0); // Top-left
        grid.set(1, 0); // Top-middle
        grid.set(2, 0); // Top-right
        grid.set(2, 1); // Middle-right

        // Middle-top cell should have 3 neighbors
        assert_eq!(grid.count_neighbors(1, 0), 3);
    }
}
