use super::{
    bounds::Bounds,
    coordinates::Coordinate,
    grid_types::{BoundedGrid, FiniteGrid, GridHeight, GridWidth},
};
use crate::grid::point::Point;

#[derive(Debug)]
pub struct FixedGrid<T> {
    inner: Vec<T>,
    width: usize,
}

impl<T> BoundedGrid for FixedGrid<T> {}
impl<T> FiniteGrid for FixedGrid<T> {
    fn bounds<Cd: Coordinate>(&self) -> Bounds<Cd> {
        Bounds {
            min_x: Cd::zero(),
            min_y: Cd::zero(),
            max_x: Cd::from_usize(self.width),
            max_y: Cd::from_usize(self.inner.len() / self.width),
        }
    }
}
impl<T, Cd: Coordinate> GridWidth<Cd> for FixedGrid<T> {
    fn width(&self) -> Cd {
        Cd::from_usize(self.width)
    }
}
impl<T, Cd: Coordinate> GridHeight<Cd> for FixedGrid<T> {
    fn height(&self) -> Cd {
        Cd::from_usize(self.height())
    }
}

impl<Cd: Coordinate, T> std::ops::Index<Point<Cd>> for FixedGrid<T> {
    type Output = T;

    fn index(&self, index: Point<Cd>) -> &Self::Output {
        let x = index.x.to_usize();
        let y = index.y.to_usize();
        debug_assert!(x < self.width);
        let idx = y * self.width + x;
        debug_assert!(idx < self.inner.len());
        &self.inner[idx]
    }
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
    pub fn height(&self) -> usize {
        self.inner.len() / self.width
    }

    pub fn get_mut_range(&mut self, start: Point<i64>, size: usize) -> &mut [T] {
        let idx = self.point_to_idx(start);
        debug_assert!(start.x as usize + size < self.width);
        &mut self.inner[idx..idx + size]
    }

    pub fn raw_iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.inner.iter()
    }
    pub fn parse_ascii_grid<F>(s: &str, parse_char: F) -> anyhow::Result<FixedGrid<T>>
    where
        F: Fn(char) -> anyhow::Result<T>,
    {
        let mut last_line = None;
        let mut line_idx = 0;
        let inner = s
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| {
                if c == '\n' {
                    let eol = idx - 1;
                    last_line = match last_line {
                        Some((line_len, line_last)) => {
                            let this_line_len = eol - line_last;
                            if this_line_len == line_len {
                                Some((line_len, idx))
                            } else {
                                return Some(Err(anyhow::anyhow!(
                                    "line no {} of len {} did not match expected len {}",
                                    line_idx,
                                    this_line_len,
                                    line_len
                                )));
                            }
                        }
                        None => Some((idx, idx)),
                    };
                    line_idx += 1;
                    None
                } else {
                    Some(parse_char(c))
                }
            })
            .collect::<anyhow::Result<Vec<T>>>()?;
        let width = last_line
            .map(|(len, _last)| len)
            .unwrap_or_else(|| inner.len());
        Ok(FixedGrid { inner, width })
    }
}

impl<T: Default> FixedGrid<T> {
    pub fn from_dimm(height: usize, width: usize) -> FixedGrid<T> {
        let mut v = Vec::with_capacity(width * height);
        v.resize_with(width * height, Default::default);
        FixedGrid { inner: v, width }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_empty_ascii_grid() {
        let g = FixedGrid::parse_ascii_grid("", |c| Ok(c)).unwrap();
        assert_eq!(g.width, 0);
    }

    #[test]
    fn build_empty_ascii_grid_no_newline() {
        let ascii = "xox\noxo\nxox";
        let g = FixedGrid::parse_ascii_grid(ascii, |c| Ok(c == 'x')).unwrap();
        assert_eq!(g.width, 3);
    }
}
