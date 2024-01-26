use macroquad::prelude::*;
use crate::{array2d::Error, cell::Cell, vector2::Vector2, Array2D};

pub struct CellGrid {
    cell_size: usize,
    array: Array2D<Cell>,
    num_updating_cells: u32,
    num_updated_cells: u32
}

impl CellGrid {
    pub fn new(size: Vector2<usize>, cell_size: usize) -> Self {
        return Self {
            cell_size,
            array: Array2D::filled_with(size, Cell::empty()),
            num_updating_cells: 0,
            num_updated_cells: 0,
        }
    }

    pub fn set(&mut self, position: &Vector2<usize>, cell: Cell) -> Result<(), Error> {
        self.array.set_at_vec(position, cell)?;
        return Ok(());
    }

    pub fn set_if_empty(&mut self, position: &Vector2<usize>, cell: Cell) {
        if let Some(old_cell) = self.array.get_from_vec(position) {
            if old_cell.empty == true {
                let _ = self.set(position, cell);
            }
        }
    }

    pub fn set_if_not_empty(&mut self, position: &Vector2<usize>, cell: Cell) {
        if let Some(old_cell) = self.array.get_from_vec(position) {
            if old_cell.empty == false {
                let _ = self.set(position, cell);
            }
        }
    }

    pub fn simulate(&mut self, delta: f32) {
        let array_ref = self.array.clone();
        let mut mut_array_ref = self.array.clone();

        let cells_to_update = mut_array_ref.iter_mut().filter(|cell| {
            if cell.empty == false {
                return true;
            };
            return false;
        });
        
        self.num_updating_cells = 0;
        self.num_updated_cells = 0;
        for cell in cells_to_update {

            let old_cell_pos = cell.position;

            let result = cell.simulate(&array_ref,delta); 
            
            if let Some(new_cell_pos) = result {
                let _ = self.set_if_not_empty(&old_cell_pos, Cell::empty());
                let _ = self.set_if_empty(&new_cell_pos, *cell);
                self.num_updated_cells += 1;
            } else {
                let _ = self.array.set_at_vec(&cell.position, *cell);
            }
            self.num_updating_cells += 1;
        }
    }

    pub fn draw(&self, debug: bool) {
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

        if debug == true {
            draw_text(format!("Cells queued to update: {}", self.num_updating_cells).as_str(), 16., 16., 16., WHITE );
            draw_text(format!("Cells updated: {}", self.num_updated_cells).as_str(), 16., 32., 16., WHITE );
        }
    }
}