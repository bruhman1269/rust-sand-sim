use macroquad::prelude::*;

mod cell;
mod cell_grid;
mod vector2;
mod array2d;

use vector2::Vector2;
use cell_grid::CellGrid;
use cell::{Cell, CellData};
use array2d::Array2D;

const SCREEN_SIZE: Vector2<usize> = Vector2::new(600, 600);
const CELL_SIZE: usize = 5;

#[macroquad::main("SandSim")]
async fn main() {
    let elements = [
        CellData::new("Sand", 0.12, 0.38, 0.55, 0.68, 60.),
        CellData::new("Gunpowder", 0.217, 0.08, 0.24, 0.38, 120.),
        CellData::new("Dead Ladybugs", 0., 1., 0., 1., -25.)
    ];

    // Create new array of cells
    let grid_size = SCREEN_SIZE.div_num(CELL_SIZE);
    let mut grid = CellGrid::new(grid_size, CELL_SIZE);
    request_new_screen_size(SCREEN_SIZE.x as f32, SCREEN_SIZE.y as f32 + 30.);

    let mut draw_size = Vector2::new(3, 3);
    let mut current_element_index = 0;

    loop {
        let delta = get_frame_time();

        clear_background(BLACK);

        if is_mouse_button_down(MouseButton::Left) || is_key_pressed(KeyCode::Space) {
            let (mouse_x, mouse_y) = mouse_position();
                let mouse_pos = Vector2::new(mouse_x as usize, mouse_y as usize).div_num(CELL_SIZE);

            for x in 0..draw_size.x {
                for y in 0..draw_size.y {
                    let position = mouse_pos.add_vector(&Vector2::new(x, y));
                    if position.x <= grid_size.x && position.y <= grid_size.y {
                        let _ = grid.set_if_empty(&position, Cell::from_cell_data(&elements[current_element_index], position));
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::R) {
            grid = CellGrid::new(SCREEN_SIZE.div_num(CELL_SIZE), CELL_SIZE);
        }

        if is_key_pressed(KeyCode::Up) {
            draw_size.add_self_vector(&Vector2::new(1usize, 1usize));
        } 

        if is_key_pressed(KeyCode::Down) {
            if draw_size.x > 1 {
                draw_size.sub_self_vector(&Vector2::new(1usize, 1usize));
            }
        }

        if is_key_pressed(KeyCode::Left) {
            if current_element_index > 0 {
                current_element_index -= 1;
            }
        }

        if is_key_pressed(KeyCode::Right) {
            if current_element_index < elements.len() - 1usize {
                current_element_index += 1;
            }
        }
        
        grid.simulate(delta);
        grid.draw(true);

        draw_text(format!("FPS: {}", get_fps()).as_str(), 16., 48., 16., GREEN);
        draw_text(format!("Draw Size: {}", draw_size.x).as_str(), 16., 64., 16., GREEN);
        draw_text(format!("Current Element: {}", elements[current_element_index].name).as_str(), 16., 80., 16., GREEN);


        
        draw_text("R to reset grid.", SCREEN_SIZE.x as f32 - 120., 16., 16., PINK);
        draw_text("Up/Down arrows to change draw size.", SCREEN_SIZE.x as f32 - 250., 32., 16., PINK);


        next_frame().await;
    }
}