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

        let delta_x = end.x - start.x;
        let delta_y = end.y - start.y;

        Self {
            delta_x,
            delta_y,
            octant,
            point: start,
            end_x: end.x,
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
        if self.point.x <= self.end_x {
            let point = self.octant.from(self.point);

            if self.error >= T::zero() {
                self.point.y += T::one();
                self.error -= self.delta_x;
            }

            self.point.x += T::one();
            self.error += self.delta_y;

            Some(point)
        } else {
            None
        }
    }
}
