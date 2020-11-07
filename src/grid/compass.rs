use super::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spin {
    Clockwise,
    AntiClockwise,
}

impl Direction {
    fn clockwise(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn anticlockwise(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    pub fn delta(self) -> Point<i64> {
        match self {
            Direction::North => Point::new(0, 1),
            Direction::East => Point::new(1, 0),
            Direction::South => Point::new(0, -1),
            Direction::West => Point::new(-1, 0),
        }
    }

    fn spin(self, spin: Spin) -> Direction {
        match spin {
            Spin::Clockwise => self.clockwise(),
            Spin::AntiClockwise => self.anticlockwise(),
        }
    }

    pub fn iter() -> CompassRose {
        CompassRose::new()
    }
}

impl IntoIterator for Direction {
    type Item = Direction;

    type IntoIter = CompassRose;

    fn into_iter(self) -> Self::IntoIter {
        CompassRose::from(self)
    }
}

pub struct CompassRose {
    start: Direction,
    inner: Option<Direction>,
}

impl CompassRose {
    fn new() -> CompassRose {
        CompassRose {
            start: Direction::North,
            inner: Some(Direction::North),
        }
    }

    fn spin_next(&mut self, spin: Spin) -> Option<Direction> {
        let d = self.inner.take();
        if let Some(d) = d {
            let n = d.spin(spin);
            if n != self.start {
                self.inner = Some(n)
            }
        }
        d
    }
}

impl From<Direction> for CompassRose {
    fn from(d: Direction) -> Self {
        CompassRose {
            start: d,
            inner: Some(d),
        }
    }
}

impl Iterator for CompassRose {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        self.spin_next(Spin::Clockwise)
    }
}

impl DoubleEndedIterator for CompassRose {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.spin_next(Spin::AntiClockwise)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compass_rose() {
        assert_eq!(
            Direction::iter().collect::<Vec<_>>(),
            vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
        )
    }

    #[test]
    fn compass_rose_reverse() {
        assert_eq!(
            Direction::iter().rev().collect::<Vec<_>>(),
            vec![
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ]
        )
    }

    #[test]
    fn offset_compass_rose() {
        let start = Direction::South;
        assert_eq!(
            start.into_iter().collect::<Vec<_>>(),
            vec![
                Direction::South,
                Direction::West,
                Direction::North,
                Direction::East,
            ]
        )
    }
}
