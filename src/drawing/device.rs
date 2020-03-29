use super::Point;
use crate::writers::{GraphicsWriter, Screen};
use core::cmp::{max, min};

pub trait Device<Color>
where
    Self: Screen + GraphicsWriter<Color>,
    Color: Clone + Copy,
{
    /// Draws a character at the given `(x, y)` coordinant to the specified `color`.
    fn draw_character(&mut self, x: usize, y: usize, character: char, color: Color);

    /// Draws a line from `start` to `end` with the specified `color`.
    fn draw_line(&mut self, start: Point<isize>, end: Point<isize>, color: Color);

    fn draw_triangle(&mut self, v0: &Point<i32>, v1: &Point<i32>, v2: &Point<i32>, color: Color) {
        let screen_width = self.get_width() as i32;
        let screen_height = self.get_height() as i32;
        let mut min_x = min(v0.x, min(v1.x, v2.x));
        let mut min_y = min(v0.y, min(v1.y, v2.y));
        let mut max_x = max(v0.x, max(v1.x, v2.x));
        let mut max_y = max(v0.y, max(v1.y, v2.y));

        min_x = max(min_x, 0);
        min_y = max(min_y, 0);
        max_x = min(max_x, screen_width - 1);
        max_y = min(max_y, screen_height - 1);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let p = Point::new(x, y);
                let w0 = orient2d(v1, v2, &p);
                let w1 = orient2d(v2, v0, &p);
                let w2 = orient2d(&v0, &v1, &p);

                if w0 >= 0 && w1 >= 0 && w2 >= 0 {
                    self.set_pixel(x as usize, y as usize, color);
                }
            }
        }
    }

    fn present(&self);
}

#[inline]
fn orient2d(a: &Point<i32>, b: &Point<i32>, c: &Point<i32>) -> i32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}
