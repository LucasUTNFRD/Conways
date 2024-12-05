mod conways;
use macroquad::prelude::*;

const GRID_WIDTH: usize = 80;
const GRID_HEIGHT: usize = 60;
const CELL_SIZE: f32 = 10.0;
const UPDATE_INTERVAL: f32 = 0.1;

#[derive(PartialEq)]
enum State {
    Running,
    Paused,
}

struct Game {
    grid: conways::Grid,
    last_update: f32,
    state: State,
}

impl Game {
    fn new() -> Self {
        let mut grid = conways::Grid::new(GRID_WIDTH, GRID_HEIGHT);
        Self::setup_glider(&mut grid);

        Self {
            grid,
            last_update: 0.0,
            state: State::Running,
        }
    }

    fn setup_glider(grid: &mut conways::Grid) {
        grid.set(10, 10, conways::CellState::Alive);
        grid.set(13, 10, conways::CellState::Alive);
        grid.set(14, 11, conways::CellState::Alive);
        grid.set(10, 12, conways::CellState::Alive);
        grid.set(14, 12, conways::CellState::Alive);
        grid.set(11, 13, conways::CellState::Alive);
        grid.set(12, 13, conways::CellState::Alive);
        grid.set(13, 13, conways::CellState::Alive);
        grid.set(14, 13, conways::CellState::Alive);
    }

    fn update(&mut self, dt: f32) {
        self.last_update += dt;

        // Update grid every UPDATE_INTERVAL seconds
        if self.last_update >= UPDATE_INTERVAL && self.state == State::Running {
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

    fn handle_input(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            self.state = match self.state {
                State::Running => State::Paused,
                State::Paused => State::Running,
            };
        }
        // if game is paused let user draw cells
        if self.state == State::Paused {
            if is_mouse_button_down(MouseButton::Left) {
                let (x, y) = (
                    mouse_position().0 as usize / CELL_SIZE as usize,
                    mouse_position().1 as usize / CELL_SIZE as usize,
                );
                self.grid.set(x, y, conways::CellState::Alive);
            }
            if is_mouse_button_down(MouseButton::Right) {
                let (x, y) = (
                    mouse_position().0 as usize / CELL_SIZE as usize,
                    mouse_position().1 as usize / CELL_SIZE as usize,
                );
                self.grid.set(x, y, conways::CellState::Dead);
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

        game.handle_input();

        game.update(dt);

        game.draw();

        next_frame().await
    }
}
