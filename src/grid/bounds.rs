use crate::grid::point::Point;
use std::cmp;

use super::{coordinates::Coordinate, DefaultCd};
#[derive(Debug, PartialEq)]
pub struct Bounds<Cd = DefaultCd> {
    pub min_x: Cd,
    pub min_y: Cd,
    pub max_x: Cd,
    pub max_y: Cd,
}

impl<Cd: Coordinate> From<Point<Cd>> for Bounds<Cd> {
    fn from(p: Point<Cd>) -> Self {
        Bounds {
            min_x: p.x,
            min_y: p.y,
            max_x: p.x,
            max_y: p.y,
        }
    }
}

impl<Cd: Coordinate> Bounds<Cd> {
    pub(crate) fn extend(&mut self, p: Point<Cd>) {
        self.min_x = cmp::min(self.min_x, p.x);
        self.min_y = cmp::min(self.min_y, p.y);
        self.max_x = cmp::max(self.max_x, p.x);
        self.max_y = cmp::max(self.max_y, p.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounds_extend() {
        let mut b = Bounds::from(Point::new(0i64, 0));
        assert_eq!(
            b,
            Bounds {
                min_x: 0,
                min_y: 0,
                max_x: 0,
                max_y: 0
            }
        );
        b.extend(Point::new(3, 5));
        assert_eq!(
            b,
            Bounds {
                min_x: 0,
                min_y: 0,
                max_x: 3,
                max_y: 5
            }
        );
        b.extend(Point::new(-1, 6));
        assert_eq!(
            b,
            Bounds {
                min_x: -1,
                min_y: 0,
                max_x: 3,
                max_y: 6
            }
        );
        b.extend(Point::new(-1, -2));
        assert_eq!(
            b,
            Bounds {
                min_x: -1,
                min_y: -2,
                max_x: 3,
                max_y: 6
            }
        );
    }
}
