use super::{
    coordinates::Coordinate,
    grid_types::{GridHeight, GridIndex, GridWidth},
};
use crate::Point;
use std::ops::Index;

pub struct HorizontalRepeat<G> {
    inner: G,
}

impl<G> HorizontalRepeat<G> {
    pub fn new(inner: G) -> HorizontalRepeat<G> {
        HorizontalRepeat { inner }
    }
}

impl<G, Cd> Index<Point<Cd>> for HorizontalRepeat<G>
where
    G: GridIndex<Cd> + GridWidth<Cd>,
    Cd: Coordinate,
{
    type Output = G::Output;

    fn index(&self, index: Point<Cd>) -> &Self::Output {
        let c_w = self.inner.width();
        let t_index = Point::new(index.x % c_w, index.y);
        self.inner.index(t_index)
    }
}

impl<Cd: Coordinate, G: GridHeight<Cd>> GridHeight<Cd> for HorizontalRepeat<G> {
    fn height(&self) -> Cd {
        self.inner.height()
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::fixed_grid::FixedGrid;
    use anyhow::Context;

    use super::*;

    #[test]
    fn index_horizontal_repeat() {
        let ascii = "123\n456\n789";
        let g = FixedGrid::parse_ascii_grid(ascii, |c| {
            let s = format!("{}", c);
            s.parse::<u8>().context(s)
        })
        .unwrap();
        let hr = HorizontalRepeat::new(g);
        for r_cycle in 0..50 {
            let mut c = 1;
            for y in 0..3 {
                let x_start = r_cycle * 3;
                let x_end = x_start + 3;
                for x in x_start..x_end {
                    assert_eq!(hr[Point::new(x, y)], c, "({}, {}) == {}", x, y, c);
                    c += 1;
                }
            }
        }
    }
}
