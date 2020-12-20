use std::ops::Index;

use crate::Point;

use super::{coordinates::Coordinate, grid_types::GridIndex};

pub trait Transform<Cd> {
    fn transform(&self, point: Point<Cd>) -> Point<Cd>;
}

struct GridTransform<G, T> {
    inner: G,
    transform: T,
}

impl<G, T, Cd> Index<Point<Cd>> for GridTransform<G, T>
where
    G: GridIndex<Cd>,
    T: Transform<Cd>,
    Cd: Coordinate,
{
    type Output = G::Output;

    fn index(&self, index: Point<Cd>) -> &Self::Output {
        let t_index = self.transform.transform(index);
        self.inner.index(t_index)
    }
}
