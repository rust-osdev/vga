use super::Point;
use crate::writers::{GraphicsWriter, Screen};
use core::cmp::{max, min};

/// A helper trait used to draw to the vga screen in graphics mode.
pub trait Device<Color>
where
    Self: Screen + GraphicsWriter<Color>,
    Color: Clone + Copy,
{
    /// Draws an 8x8 character at the given `(x, y)` coordinant to the specified `color`.
    ///
    /// **Note:** This does no bounds checking and will panick if
    /// any of the pixels fall outside of the screen range.
    /// `x + 8 >= self.get_width() || y + 8 >= self.get_height()`.
    fn draw_character(&mut self, x: usize, y: usize, character: char, color: Color);

    /// Draws a line from `start` to `end` with the specified `color`.
    ///
    /// **Note:** This does no bounds checking and will panick if
    /// `x >= self.get_width() || y >= self.get_height()`.
    fn draw_line(&mut self, start: Point<isize>, end: Point<isize>, color: Color);

    /// Draws a triangle to the screen with the given points `(v0, v1, v2)`
    /// and the given `color`.
    ///
    /// **Note:** This function will clip any pixels that are
    /// not contained within the screen coordinates.
    /// `x < 0 || x >= self.get_width() || y < 0 || y >= self.get_height()`.
    fn draw_triangle(&mut self, v0: Point<i32>, v1: Point<i32>, v2: Point<i32>, color: Color) {
        let screen_width = self.get_width() as i32;
        let screen_height = self.get_height() as i32;
        let (a01, b01) = (v0.y - v1.y, v1.x - v0.x);
        let (a12, b12) = (v1.y - v2.y, v2.x - v1.x);
        let (a20, b20) = (v2.y - v0.y, v0.x - v2.x);

        let mut min_x = min(v0.x, min(v1.x, v2.x));
        let mut min_y = min(v0.y, min(v1.y, v2.y));
        let mut max_x = max(v0.x, max(v1.x, v2.x));
        let mut max_y = max(v0.y, max(v1.y, v2.y));

        min_x = max(min_x, 0);
        min_y = max(min_y, 0);
        max_x = min(max_x, screen_width - 1);
        max_y = min(max_y, screen_height - 1);

        let p = Point::new(min_x, min_y);
        let mut w0_row = orient2d(v1, v2, p);
        let mut w1_row = orient2d(v2, v0, p);
        let mut w2_row = orient2d(v0, v1, p);

        for y in p.y..=max_y {
            let mut w0 = w0_row;
            let mut w1 = w1_row;
            let mut w2 = w2_row;

            for x in p.x..=max_x {
                if (w0 | w1 | w2) >= 0 {
                    self.set_pixel(x as usize, y as usize, color);
                }

                w0 += a12;
                w1 += a20;
                w2 += a01;
            }

            w0_row += b12;
            w1_row += b20;
            w2_row += b01;
        }
    }

    /// Copies the screen buffer in the `GraphicsWriter` to vga memory.
    ///
    /// **Note:** No draw calls will be displayed on the screen unless
    /// this method is called.
    fn present(&self);
}

#[inline]
fn orient2d(a: Point<i32>, b: Point<i32>, c: Point<i32>) -> i32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}
