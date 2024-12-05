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
        grid.set_alive(2, 1);
        grid.set_alive(3, 2);
        grid.set_alive(1, 3);
        grid.set_alive(2, 3);
        grid.set_alive(3, 3);

        // Draw a heavy weight spaceship
        grid.set_alive(10, 10);
        grid.set_alive(13, 10);
        grid.set_alive(14, 11);
        grid.set_alive(10, 12);
        grid.set_alive(14, 12);
        grid.set_alive(11, 13);
        grid.set_alive(12, 13);
        grid.set_alive(13, 13);
        grid.set_alive(14, 13);
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
                self.grid.set_alive(x, y);
            }
            if is_mouse_button_down(MouseButton::Right) {
                let (x, y) = (
                    mouse_position().0 as usize / CELL_SIZE as usize,
                    mouse_position().1 as usize / CELL_SIZE as usize,
                );
                self.grid.set_dead(x, y);
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
