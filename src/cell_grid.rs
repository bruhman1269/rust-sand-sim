use macroquad::prelude::*;
use crate::{cell::Cell, vector2::Vector2, Array2D};

pub struct CellGrid {
    cell_size: usize,
    array: Array2D<Cell>
}

impl CellGrid {
    pub fn new(size: Vector2<usize>, cell_size: usize) -> Self {
        return Self {
            cell_size,
            array: Array2D::filled_with(size, Cell::empty())
        }
    }

    pub fn set(&mut self, position: &Vector2<usize>, cell: Cell) {
        self.array.set_at_vec(position, cell);
    }

    pub fn simulate(&mut self) -> Option<()> {
        for cell in self.array.iter_mut() {
            let bottom_pos = cell.position.add_vector(&Vector2::new(0, 1));
            let bottom_cell = self.array.get_from_vec(&bottom_pos)?;

            if bottom_cell.empty == true {
                
            }
        }

        return Some(());
    }

    pub fn draw(&self) {
        for cell in self.array.iter() {
            let cell_size = self.cell_size as f32;
            let cell_pos_x = cell.position.x as f32;
            let cell_pos_y = cell.position.y as f32;

            draw_rectangle(
                cell_pos_x * cell_size,
                cell_pos_y * cell_size,
                cell_size,
                cell_size,
                cell.color
            );
        }
    }
}