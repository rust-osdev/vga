//! Common functionality for drawing in vga graphics mode.
//! Original implementation here https://github.com/expenses/line_drawing.
use num_traits::{NumAssignOps, NumCast, Signed};

mod bresenham;
mod device;
mod octant;

pub(crate) use bresenham::Bresenham;
pub use device::Device;
use octant::Octant;

/// A point in 2D space.
#[derive(Copy, Clone)]
pub struct Point<T> {
    /// The x coordinate of the `Point`.
    pub x: T,
    /// The y coordinate of the `Point`.
    pub y: T,
}

impl<T> Point<T> {
    /// Creates a new `Point` with the given `(x, y)` coordinates.
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

pub(crate) trait SignedNum: Signed + Ord + Copy + NumCast + NumAssignOps {
    fn cast<T: NumCast>(value: T) -> Self {
        NumCast::from(value).unwrap()
    }
}

impl<T: Signed + Ord + Copy + NumCast + NumAssignOps> SignedNum for T {}
