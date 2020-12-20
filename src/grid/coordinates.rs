use std::{hash::Hash, ops::Rem};

pub trait Zero {
    fn zero() -> Self;
}

pub trait FromUsize {
    fn from_usize(u: usize) -> Self;
    fn to_usize(self) -> usize;
}

pub trait Coordinate: Ord + Hash + Copy + Zero + FromUsize + Rem<Output = Self> {}

impl<T> Coordinate for T where T: Ord + Hash + Copy + Zero + FromUsize + Rem<Output = T> {}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
}
impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}
impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}
impl Zero for u64 {
    fn zero() -> Self {
        0
    }
}
impl Zero for usize {
    fn zero() -> Self {
        0
    }
}
impl FromUsize for i32 {
    fn from_usize(u: usize) -> Self {
        u as i32
    }
    fn to_usize(self) -> usize {
        debug_assert!(self >= 0);
        self as usize
    }
}
impl FromUsize for i64 {
    fn from_usize(u: usize) -> Self {
        u as i64
    }
    fn to_usize(self) -> usize {
        debug_assert!(self >= 0);
        self as usize
    }
}
impl FromUsize for u32 {
    fn from_usize(u: usize) -> Self {
        u as u32
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}
impl FromUsize for u64 {
    fn from_usize(u: usize) -> Self {
        u as u64
    }
    fn to_usize(self) -> usize {
        self as usize
    }
}
impl FromUsize for usize {
    fn from_usize(u: usize) -> Self {
        u
    }
    fn to_usize(self) -> usize {
        self
    }
}
