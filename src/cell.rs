use macroquad::color::{self, colors, Color};
use rand::Rng;
use crate::{array2d::{Array2D, Error}, vector2::Vector2};
pub struct CellData {
    pub name: &'static str,
    hue: f32,
    saturation: f32,
    lightness_range_lower: f32,
    lightness_range_upper: f32,
    falling_speed: f32,
}

impl CellData {
    pub fn new(name: &'static str, hue: f32, saturation: f32, lr_lower: f32, lr_upper: f32, falling_speed: f32) -> Self {
        return Self {
            name,
            hue,
            saturation,
            falling_speed,
            lightness_range_lower: lr_lower,
            lightness_range_upper: lr_upper
        };
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub color: Color,
    pub position: Vector2<usize>,
    pub empty: bool,
    falling_speed: f32,
    subposition: f32,
    should_fall_down: bool
}

impl Cell {

    pub fn new(position: Vector2<usize>, hue: f32, saturation: f32, lr_lower: f32, lr_upper: f32, falling_speed: f32) -> Self {
        let mut rng = rand::thread_rng();
        return Self {
            position,
            color: color::hsl_to_rgb(hue, saturation, rng.gen_range(lr_lower..lr_upper)),
            falling_speed,
            empty: false,
            subposition: 0.,
            should_fall_down: falling_speed > 0.
        };
    }

    pub fn from_cell_data(cell_data: &CellData, position: Vector2<usize>) -> Self {
        return Self::new(
            position,
            cell_data.hue,
            cell_data.saturation,
            cell_data.lightness_range_lower,
            cell_data.lightness_range_upper,
            cell_data.falling_speed
        );
    }

    pub fn empty() -> Self {
        return Self {
            color: colors::BLACK,
            position: Vector2::new(0, 0),
            empty: true,
            falling_speed: 0.,
            subposition: 0.,
            should_fall_down: false
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

        if self.should_fall_down == true {
            self.fall_down(array_ref, delta);
        } else {
            self.float_up(array_ref);
        }

        if previous_position != self.position {
            return Some(self.position);
        } else {
            return None
        }
    }

    fn fall_down(&mut self, array_ref: &Array2D<Cell>, delta: f32) {
        let lower_cell: &Cell;
        if let Some(cell) = array_ref.get_from_vec(&self.position.add_vector(&Vector2::new(0usize, 1usize))) {
            lower_cell = cell;
        } else {
            self.subposition = 0.;
            return;
        }
        
        self.subposition += self.falling_speed * delta;

        if self.subposition > 1. {
            let subpos_floor = f32::floor(self.subposition);
            let subpos_floor_usize = subpos_floor as usize;

            // If there's a cell directly below (try to fall to sides)
            if lower_cell.empty == false {
                let mut left_cell_empty = false;
                let mut right_cell_empty = false;

                if self.position.x > 0 {
                    if let Some(left_cell) = array_ref.get_from_vec(&Vector2::new(self.position.x - 1usize, self.position.y + 1usize)) {
                        left_cell_empty = left_cell.empty == true;
                    }
                }

                if self.position.x < array_ref.size.x - 1usize {
                    if let Some(right_cell) = array_ref.get_from_vec(&Vector2::new(self.position.x + 1, self.position.y + 1)) {
                        right_cell_empty = right_cell.empty == true;
                    }
                }

                if left_cell_empty && right_cell_empty {
                    let go_left = rand::random::<bool>();
                    if go_left == true {
                        self.position.x -= 1usize;
                    } else {
                        self.position.x += 1usize;
                    }
                } else if left_cell_empty {
                    self.position.x -= 1usize;
                } else if right_cell_empty {
                    self.position.x += 1usize;
                }

                if left_cell_empty || right_cell_empty {
                    self.raycast_down_and_change_pos(array_ref, subpos_floor_usize);
                }
                
            } else {
                self.raycast_down_and_change_pos(array_ref, subpos_floor_usize);
            }

            self.subposition -= subpos_floor;
        }
    }

    fn float_up(&mut self, array_ref: &Array2D<Cell>) {
        if self.subposition < 1. {
            let subpos_floor = f32::floor(self.subposition);
            let subpos_floor_usize = subpos_floor as usize;

            if let Some(result) = self.raycast_up(array_ref, subpos_floor_usize) {
                if let Ok(upper_cell) = result {
                    self.position = upper_cell.position.add_vector(&Vector2::new(0usize, 1usize));
                } else {
                    self.position.sub_self_vector(&Vector2::new(0usize, subpos_floor_usize))
                }
            }
            self.subposition -= subpos_floor;
        }
    }

    fn raycast_down_and_change_pos(&mut self, array_ref: &Array2D<Cell>, subpos_floor_usize: usize) {
        if let Some(result) = self.raycast_down(array_ref, subpos_floor_usize) {
            if let Ok(lower_cell) = result { 
                self.position = lower_cell.position.sub_vector(&Vector2::new(0usize, 1usize));
            } else {
                self.position = Vector2::new(self.position.x, array_ref.size.y - 1usize);
            }
        } else {
            self.position.add_self_vector(&Vector2::new(0usize, subpos_floor_usize));
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

    fn raycast_up<'a>(&self, array_ref: &'a Array2D<Cell>, distance: usize) -> Option<Result<&'a Cell, Error>>{
        for distance_index in 1..=distance {
            //draw_rectangle((self.position.x * CELL_SIZE) as f32, ((self.position.y + distance_index + 1) * CELL_SIZE) as f32, CELL_SIZE as f32, CELL_SIZE as f32, RED);
            if let Some(upper_cell) = array_ref.get_from_vec(
                &self.position.sub_vector(&Vector2::new(0usize, distance_index))
            ) {
                if upper_cell.empty == false {
                    return Some(Ok(upper_cell));
                }
            } else {
                return Some(Err(Error::IndexOutOfBounds));
            }
        }

        return None;
    }
}