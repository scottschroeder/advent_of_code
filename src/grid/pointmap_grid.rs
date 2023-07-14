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
    pub fn bounds(&self) -> Option<&Bounds<Cd>> {
        self.bounds.as_ref()
    }
    pub fn get(&self, key: &Point<Cd>) -> Option<&T> {
        self.inner.get(key)
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

impl<T: std::fmt::Display> std::fmt::Display for PointMap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bounds = match &self.bounds {
            Some(b) => b,
            None => return Ok(()),
        };

        for idy in bounds.min_y..(bounds.max_y + 1) {
            for idx in bounds.min_x..(bounds.max_x + 1) {
                if let Some(e) = self.inner.get(&Point::new(idx, idy)) {
                    write!(f, "{}", e)?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
