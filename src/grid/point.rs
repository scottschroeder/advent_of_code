use std::fmt;
use std::ops::{Add, Mul, Sub};
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Scalar<T>(pub T);

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add> Add for Point<T> {
    type Output = Point<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub> Sub for Point<T> {
    type Output = Point<<T as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Mul + Copy> Mul<Point<T>> for Scalar<T> {
    type Output = Point<<T as Mul>::Output>;

    fn mul(self, rhs: Point<T>) -> Self::Output {
        let Point { x, y } = rhs;
        let scalar = self.0;
        Point::new(scalar * x, scalar * y)
    }
}

impl<T: fmt::Debug> fmt::Debug for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}
impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from(t: (T, T)) -> Self {
        Point::new(t.0, t.1)
    }
}

impl<T: Copy> From<&(T, T)> for Point<T> {
    fn from(t: &(T, T)) -> Self {
        Point::new(t.0, t.1)
    }
}

impl<T: Copy> From<&Point<T>> for Point<T> {
    fn from(p: &Point<T>) -> Self {
        *p
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Point::new(7, 10), Point::new(2, 3) + Point::new(5, 7))
    }
    #[test]
    fn sub_pos() {
        assert_eq!(Point::new(2, 3), Point::new(7, 10) - Point::new(5, 7))
    }
    #[test]
    fn sub_neg() {
        assert_eq!(Point::new(-3, -4), Point::new(2, 3) - Point::new(5, 7))
    }
    #[test]
    #[should_panic(expected = "subtract with overflow")]
    fn sub_neg_underflow() {
        type UPoint = Point<usize>;
        let _ = UPoint::new(2, 3) - UPoint::new(5, 7);
    }
}
