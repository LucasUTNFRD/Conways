mod conways;
use macroquad::prelude::*;

const GRID_WIDTH: usize = 80;
const GRID_HEIGHT: usize = 60;
const CELL_SIZE: f32 = 10.0;
const UPDATE_INTERVAL: f32 = 0.1;

struct Game {
    grid: conways::Grid,
    last_update: f32,
}

impl Game {
    fn new() -> Self {
        let mut grid = conways::Grid::new(GRID_WIDTH, GRID_HEIGHT);
        Self::setup_glider(&mut grid);

        Self {
            grid,
            last_update: 0.0,
        }
    }

    fn setup_glider(grid: &mut conways::Grid) {
        grid.set_alive(2, 1);
        grid.set_alive(3, 2);
        grid.set_alive(1, 3);
        grid.set_alive(2, 3);
        grid.set_alive(3, 3);
    }

    fn update(&mut self, dt: f32) {
        self.last_update += dt;

        // Update grid every UPDATE_INTERVAL seconds
        if self.last_update >= UPDATE_INTERVAL {
            self.grid.next_cell_generation();
            self.last_update = 0.0;
        }
    }

    fn draw(&self) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if self.grid.get(x, y) == conways::CellState::Alive {
                    draw_rectangle(
                        x as f32 * CELL_SIZE,
                        y as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        WHITE,
                    );
                }
            }
        }
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    // set a loop where i can see the test_glider
    let mut game = Game::new();
    loop {
        clear_background(BLACK);

        let dt = get_frame_time();

        game.update(dt);

        game.draw();

        next_frame().await
    }
}
