use super::{Octant, Point, SignedNum};

pub(crate) struct Bresenham<T> {
    point: Point<T>,
    end_x: T,
    delta_x: T,
    delta_y: T,
    error: T,
    octant: Octant,
}

impl<T: SignedNum> Bresenham<T> {
    #[inline]
    pub fn new(start: Point<T>, end: Point<T>) -> Self {
        let octant = Octant::new(start, end);
        let start = octant.to(start);
        let end = octant.to(end);

        let delta_x = end.0 - start.0;
        let delta_y = end.1 - start.1;

        Self {
            delta_x,
            delta_y,
            octant,
            point: start,
            end_x: end.0,
            error: delta_y - delta_x,
        }
    }
}

impl<T> Iterator for Bresenham<T>
where
    T: SignedNum,
{
    type Item = Point<T>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.point.0 <= self.end_x {
            let point = self.octant.from(self.point);

            if self.error >= T::zero() {
                self.point.1 += T::one();
                self.error -= self.delta_x;
            }

            self.point.0 += T::one();
            self.error += self.delta_y;

            Some(point)
        } else {
            None
        }
    }
}
