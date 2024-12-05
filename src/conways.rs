#[derive(Clone, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

pub struct Grid {
    grid: Vec<Vec<CellState>>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![CellState::Dead; width]; height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.grid[y][x] = CellState::Alive;
    }

    pub fn get(&self, x: usize, y: usize) -> CellState {
        self.grid[y][x].clone()
    }
}
