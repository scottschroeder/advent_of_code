use std::ops::Index;

use super::{bounds::Bounds, coordinates::Coordinate, point::Point};

pub trait GridIndex<Cd>: Index<Point<Cd>> {}

// impl<Cd: Coordinate, T: GridIndex<Cd>> Index<Point<Cd>> for T {
//     type Output=<T as GridIndex<Cd>>::Output;

//     fn index(&self, index: Point<Cd>) -> &Self::Output {
//         self.grid_index(index)
//     }
// }

impl<Cd: Coordinate, T: Index<Point<Cd>>> GridIndex<Cd> for T {}

pub trait FiniteGrid {
    fn bounds<Cd: Coordinate>(&self) -> Bounds<Cd>;
}

pub trait GridWidth<Cd: Coordinate> {
    fn width(&self) -> Cd;
}
pub trait GridHeight<Cd: Coordinate> {
    fn height(&self) -> Cd;
}

pub trait BoundedGrid {}
