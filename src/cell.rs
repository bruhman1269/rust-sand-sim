use macroquad::color::{self, colors, Color};
use rand::Rng;
use crate::{array2d::Array2D, vector2::Vector2};

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub color: Color,
    pub position: Vector2<usize>,
    pub empty: bool,
    falling_speed: f32,
    pub subposition: f32
}

impl Cell {
    pub fn new(position: Vector2<usize>, falling_speed: f32) -> Self {
        let mut rng = rand::thread_rng();
        return Self {
            position,
            color: color::hsl_to_rgb(0.15, 0.48, rng.gen_range(0.34..0.57)),
            falling_speed,
            empty: false,
            subposition: 0.
        };
    }

    pub fn empty() -> Self {
        return Self {
            color: colors::BLACK,
            position: Vector2::new(0, 0),
            empty: true,
            falling_speed: 0.,
            subposition: 0.
        };
    }

    // pub fn copy_with_pos(cell: &Cell, position: Vector2<usize>) -> Self {
    //     return Cell {
    //         position,
    //         color: cell.color,
    //         empty: cell.empty,
    //         falling_speed: cell.falling_speed,
    //         subposition: 0.
    //     }
    // }

    pub fn simulate(&mut self, array_ref: &Array2D<Cell>, delta: f32) -> Option<Vector2<usize>> {
        // Check if there's a cell below
        if let Some(bottom_cell) = array_ref.get_from_vec(
            &self.position.add_vector(&Vector2::new(0usize, 1usize))
        ) {
            if bottom_cell.empty == false {
                return None;
            }
        } else {
            return None;
        }
        
        let previous_position = self.position;
        self.subposition += self.falling_speed * delta;
        
        if self.subposition > 1. {
            let subpos_floor = f32::floor(self.subposition);
            self.position.add_self_vector(&Vector2::new(0 as usize, subpos_floor as usize));
            self.subposition -= subpos_floor;
        }

        if previous_position != self.position {
            return Some(self.position);
        } else {
            return None
        }
    }
}