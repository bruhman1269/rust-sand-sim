use std::fmt::Debug;
use std::slice::{Iter, IterMut};

use crate::Vector2;

#[derive(Debug)]
pub enum Error {
    IndexOutOfBounds
}

#[derive(Clone)]
pub struct Array2D<T> {
    pub size: Vector2<usize>,
    array: Vec<T>,
    total_size: usize
}

impl<T: Copy + Debug> Array2D<T> {
    // pub fn empty() -> Self {
    //     return Self {
    //         size: Vector2::new(0, 0),
    //         array: vec![],
    //         total_size: 0
    //     };
    // }

    pub fn filled_with(size: Vector2<usize>, element: T) -> Self {
        return Self {
            size,
            array: vec![element; size.x * size.y],
            total_size: size.x * size.y
        }
    }

    pub fn get_from_vec(&self, vec: &Vector2<usize>) -> Option<&T> {
        return self.array.get(self.index_from_vec(vec));
    }

    pub fn set_at_vec(&mut self, position: &Vector2<usize>, element: T) -> Result<(), Error> {
        let index = self.index_from_vec(position);
        
        if index >= self.total_size {
            return Err(Error::IndexOutOfBounds);
        } 

        self.array[index] = element;
        return Ok(());
    }

    fn index_from_vec(&self, vector: &Vector2<usize>) -> usize {
        return vector.x + vector.y * self.size.x;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        return self.array.iter();
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        return self.array.iter_mut();
    }
}