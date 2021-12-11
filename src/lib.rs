pub mod grid {
    mod bounds;
    pub mod compass;
    mod coordinates;
    pub mod fixed_grid;
    pub mod grid_types;
    pub mod point;
    pub mod pointmap_grid;
    pub mod repeat_grid;
    mod transform;

    type DefaultCd = i64;
}
pub mod ac3;
pub mod bitset;
pub use self::error::Error;
pub use crate::grid::point::Point;

pub mod math {

    // Returns None when the two numbers are not coprime (numbers that share no prime factors)
    pub fn inverse_mod(a: i64, n: i64) -> Option<i64> {
        // Also see Chinese Remainder Theorem
        let mut mn = (n, a);
        let mut xy = (0, 1);

        while mn.1 != 0 {
            xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
            mn = (mn.1, mn.0 % mn.1);
        }

        if mn.0 > 1 {
            return None;
        }

        while xy.0 < 0 {
            xy.0 += n;
        }
        Some(xy.0)
    }
}
mod error {
    use std::fmt;
    pub struct Error(pub anyhow::Error);
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)?;
            for cause in self.0.chain().skip(1) {
                write!(f, ": {}", cause)?
            }
            Ok(())
        }
    }
}
