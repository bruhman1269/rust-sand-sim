use macroquad::color::{Color, colors};

use crate::{array2d::Array2D, vector2::Vector2};

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub color: Color,
    pub position: Vector2<usize>,
    pub empty: bool
}

impl Cell {
    pub fn new(position: Vector2<usize>, color: Color) -> Self {
        return Self { position, color, empty: false };
    }

    pub fn empty() -> Self {
        return Self {
            color: colors::BLACK,
            position: Vector2::new(0, 0),
            empty: true
        };
    }
}