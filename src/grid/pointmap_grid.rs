use crate::grid::{bounds::Bounds, point::Point};
use std::collections::HashMap;

use super::{coordinates::Coordinate, DefaultCd};

#[derive(Debug)]
pub struct PointMap<T, Cd = DefaultCd> {
    inner: HashMap<Point<Cd>, T>,
    bounds: Option<Bounds<Cd>>,
}

impl<Cd: Coordinate, T> PointMap<T, Cd> {
    pub fn insert(&mut self, p: Point<Cd>, value: T) -> Option<T> {
        self.bounds.get_or_insert_with(|| Bounds::from(p)).extend(p);
        self.inner.insert(p, value)
    }
}

impl<T> Default for PointMap<T> {
    fn default() -> Self {
        PointMap {
            inner: HashMap::default(),
            bounds: None,
        }
    }
}
