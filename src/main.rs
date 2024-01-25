use macroquad::prelude::*;

mod cell;
mod cell_grid;
mod vector2;
mod array2d;

use vector2::Vector2;
use cell_grid::CellGrid;
use cell::Cell;
use array2d::Array2D;

const SCREEN_SIZE: Vector2<usize> = Vector2::new(600, 600);
const CELL_SIZE: usize = 5;

#[macroquad::main("SandSim")]
async fn main() {

    // Create new array of cells
    let mut grid = CellGrid::new(SCREEN_SIZE.div_num(CELL_SIZE), CELL_SIZE);
    
    request_new_screen_size(SCREEN_SIZE.x as f32, SCREEN_SIZE.y as f32);

    loop {
        clear_background(BLACK);

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let mouse_pos = Vector2::new(mouse_x as usize, mouse_y as usize).div_num(CELL_SIZE);
            let _ = grid.set(&mouse_pos, Cell::new(mouse_pos, YELLOW));
        }

        grid.simulate();
        grid.draw();

        next_frame().await;
    }
}