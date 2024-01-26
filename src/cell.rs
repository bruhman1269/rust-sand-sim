use macroquad::color::{self, colors, Color};
use rand::Rng;
use crate::{array2d::{Array2D, Error}, vector2::Vector2};

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
            color: color::hsl_to_rgb(0.15, 0.48, rng.gen_range(0.57..0.78)),
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

    // pub fn empty_with_pos(position: Vector2<usize>) -> Self {
    //     let mut new_cell = Self::empty();
    //     new_cell.position = position;
    //     return new_cell;
    // }

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
        let previous_position = self.position;
        self.subposition += self.falling_speed * delta;
        
        if self.subposition > 1. {
            let subpos_floor = f32::floor(self.subposition);
            let subpos_floor_usize = subpos_floor as usize;

            if let Some(result) = self.raycast_down(array_ref, subpos_floor_usize) {
                if let Ok(lower_cell) = result { 
                    self.position = lower_cell.position.sub_vector(&Vector2::new(0usize, 1usize));
                } else {
                    self.position = Vector2::new(self.position.x, array_ref.size.y - 1usize);
                }
            } else {
                self.position.add_self_vector(&Vector2::new(0 as usize, subpos_floor_usize));
                self.subposition -= subpos_floor;                
            }
        }

        if previous_position != self.position {
            return Some(self.position);
        } else {
            return None
        }
    }

    fn raycast_down<'a>(&self, array_ref: &'a Array2D<Cell>, distance: usize) -> Option<Result<&'a Cell, Error>>{
        for distance_index in 1..=distance {
            //draw_rectangle((self.position.x * CELL_SIZE) as f32, ((self.position.y + distance_index + 1) * CELL_SIZE) as f32, CELL_SIZE as f32, CELL_SIZE as f32, RED);
            if let Some(lower_cell) = array_ref.get_from_vec(
                &self.position.add_vector(&Vector2::new(0usize, distance_index))
            ) {
                if lower_cell.empty == false {
                    return Some(Ok(lower_cell));
                }
            } else {
                return Some(Err(Error::IndexOutOfBounds));
            }
        }

        return None;
    }
}