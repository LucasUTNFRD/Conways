mod conways;
use macroquad::prelude::*;

//define const for the grid size
const GRID_WIDTH: usize = 80;
const GRID_HEIGHT: usize = 60;

fn conf() -> Conf {
    Conf {
        window_title: "Conway's Game of Life".to_string(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

fn test_glider(grid: &mut conways::Grid) {
    grid.set(1, 2);
    grid.set(2, 3);
    grid.set(3, 1);
    grid.set(3, 2);
    grid.set(3, 3);
    grid.set(3, 5);
}

#[macroquad::main(conf)]
async fn main() {
    let mut grid = conways::Grid::new(GRID_WIDTH, GRID_HEIGHT);

    test_glider(&mut grid);

    // set a loop where i can see the test_glider
    loop {
        clear_background(BLACK);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                // this should be a get_alive cell from grid
                if grid.get(x, y) == conways::CellState::Alive {
                    // This draws the cell
                    draw_rectangle(x as f32 * 10.0, y as f32 * 10.0, 10.0, 10.0, WHITE);
                }
            }
        }

        next_frame().await
    }
}
