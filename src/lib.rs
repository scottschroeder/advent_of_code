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
pub mod bitset;
pub use self::error::Error;
pub use crate::grid::point::Point;

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
