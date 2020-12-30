use std::{
    fmt,
    ops::{Add, Sub},
};

use super::DefaultCd;
// #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct Scalar<T>(pub T);

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point<T = DefaultCd> {
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

impl Point<i64> {
    pub fn adjacent_all(self) -> AdjacentPoints {
        AdjacentPoints {
            center: self,
            counter: 0,
            include_center: false,
            include_rook: true,
            include_bishop: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Transform {
    NorthWest,
    North,
    NorthEast,
    West,
    None,
    East,
    SouthWest,
    South,
    SouthEast,
}

enum DirectionType {
    None,
    Rook,
    Bishop,
}

const ADJACENT: [Transform; 9] = [
    Transform::NorthWest,
    Transform::North,
    Transform::NorthEast,
    Transform::West,
    Transform::None,
    Transform::East,
    Transform::SouthWest,
    Transform::South,
    Transform::SouthEast,
];

impl Transform {
    fn direction_type(self) -> DirectionType {
        match self {
            Transform::NorthWest => DirectionType::Bishop,
            Transform::North => DirectionType::Rook,
            Transform::NorthEast => DirectionType::Bishop,
            Transform::West => DirectionType::Rook,
            Transform::None => DirectionType::None,
            Transform::East => DirectionType::Rook,
            Transform::SouthWest => DirectionType::Bishop,
            Transform::South => DirectionType::Rook,
            Transform::SouthEast => DirectionType::Bishop,
        }
    }
    fn transform(self, p: Point<i64>) -> Point<i64> {
        match self {
            Transform::NorthWest => Point::new(p.x - 1, p.y - 1),
            Transform::North => Point::new(p.x, p.y - 1),
            Transform::NorthEast => Point::new(p.x + 1, p.y - 1),
            Transform::West => Point::new(p.x - 1, p.y),
            Transform::None => Point::new(p.x, p.y),
            Transform::East => Point::new(p.x + 1, p.y),
            Transform::SouthWest => Point::new(p.x - 1, p.y + 1),
            Transform::South => Point::new(p.x, p.y + 1),
            Transform::SouthEast => Point::new(p.x + 1, p.y + 1),
        }
    }
}

pub struct AdjacentPoints {
    center: Point<i64>,
    counter: usize,
    include_center: bool,
    include_rook: bool,
    include_bishop: bool,
}

impl Iterator for AdjacentPoints {
    type Item = Point<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.counter < ADJACENT.len() {
            let transform = ADJACENT[self.counter];
            self.counter += 1;
            match transform.direction_type() {
                DirectionType::None => {
                    if !self.include_center {
                        continue;
                    }
                }
                DirectionType::Rook => {
                    if !self.include_rook {
                        continue;
                    }
                }
                DirectionType::Bishop => {
                    if !self.include_bishop {
                        continue;
                    }
                }
            }
            return Some(transform.transform(self.center));
        }
        None
    }
}

// impl<T: Mul + Copy> Mul<Point<T>> for Scalar<T> {
//     type Output = Point<<T as Mul>::Output>;

//     fn mul(self, rhs: Point<T>) -> Self::Output {
//         let Point { x, y } = rhs;
//         let scalar = self.0;
//         Point::new(scalar * x, scalar * y)
//     }
// }

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

    #[test]
    fn adjacent_0() {
        let p = Point::new(0i64, 0);
        assert_eq!(
            p.adjacent_all().collect::<Vec<_>>(),
            vec![
                (-1, -1).into(),
                (0, -1).into(),
                (1, -1).into(),
                (-1, 0).into(),
                (1, 0).into(),
                (-1, 1).into(),
                (0, 1).into(),
                (1, 1).into(),
            ]
        )
    }
}
