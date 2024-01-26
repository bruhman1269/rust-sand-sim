use std::ops::{Add, AddAssign, Div, DivAssign, SubAssign, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        return Self { x, y };
    }

    // pub fn from_tuple(tuple: (T, T)) -> Self {
    //     return Self {
    //         x: tuple.0,
    //         y: tuple.1
    //     }
    // }
}

impl<T: DivAssign + Div<Output = T> + AddAssign + Add<Output = T> + SubAssign + Sub<Output = T>+ Copy> Vector2<T> {

    // pub fn div_self_vec(&mut self, other: &Vector2<T>) {
    //     self.x /= other.x;
    //     self.y /= other.y;
    // }

    // pub fn div_self_num(&mut self, num: T) {
    //     self.x /= num;
    //     self.y /= num;
    // }

    pub fn add_self_vector(&mut self, other: &Vector2<T>) {
        self.x += other.x;
        self.y += other.y;
    }

    // pub fn div_vec(&self, other: Vector2<T>) -> Self {
    //     let x = self.x / other.x;
    //     let y = self.y / other.y;
    //     return Self { x, y };
    // }

    pub fn div_num(&self, num: T) -> Self {
        let x = self.x / num;
        let y = self.y / num;
        return Self { x, y };
    }


    pub fn add_vector(&self, vector: &Vector2<T>) -> Self {
        return Self {
            x: self.x + vector.x,
            y: self.y + vector.y
        };
    }
}

impl<T: PartialEq> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}