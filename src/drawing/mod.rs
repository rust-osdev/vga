//! Common functionality for drawing in vga graphics mode.
//! Original implementation here https://github.com/expenses/line_drawing.
use num_traits::{NumAssignOps, NumCast, Signed};

mod bresenham;
mod octant;

pub(crate) use bresenham::Bresenham;
use octant::Octant;

/// A point in 2D space.
pub type Point<T> = (T, T);

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rectangle {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Rectangle {
        Rectangle {
            x,
            y,
            width,
            height,
        }
    }

    pub fn bottom(&self) -> usize {
        self.y + self.height
    }

    pub fn right(&self) -> usize {
        self.x + self.width
    }
}

pub(crate) trait SignedNum: Signed + Ord + Copy + NumCast + NumAssignOps {
    fn cast<T: NumCast>(value: T) -> Self {
        NumCast::from(value).unwrap()
    }
}

impl<T: Signed + Ord + Copy + NumCast + NumAssignOps> SignedNum for T {}
