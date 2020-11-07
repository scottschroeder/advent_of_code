use crate::grid::point::Point;

#[derive(Debug)]
pub struct FixedGrid<T> {
    inner: Vec<T>,
    width: usize,
}

impl<T> FixedGrid<T> {
    pub fn point_to_idx(&self, p: Point<i64>) -> usize {
        debug_assert!(p.x > 0);
        debug_assert!(p.y > 0);
        let x = p.x as usize;
        let y = p.y as usize;
        debug_assert!(x < self.width);
        let idx = y * self.width + x;
        debug_assert!(idx < self.inner.len());
        idx
    }
    pub fn idx_to_point(&self, idx: usize) -> Point<i64> {
        debug_assert!(idx < self.inner.len());
        let x = idx % self.width;
        let y = idx / self.width;
        Point::new(x as i64, y as i64)
    }

    pub fn get_mut_range(&mut self, start: Point<i64>, size: usize) -> &mut [T] {
        let idx = self.point_to_idx(start);
        debug_assert!(start.x as usize + size < self.width);
        &mut self.inner[idx..idx + size]
    }

    pub fn raw_iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.inner.iter()
    }
}

impl<T: Default> FixedGrid<T> {
    pub fn from_dimm(height: usize, width: usize) -> FixedGrid<T> {
        let mut v = Vec::with_capacity(width * height);
        v.resize_with(width * height, Default::default);
        FixedGrid { inner: v, width }
    }
}
