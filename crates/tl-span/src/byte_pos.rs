use std::ops::{Add, Sub};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytePos(u32);

impl BytePos {
    pub const fn new(pos: u32) -> Self {
        Self(pos)
    }

    pub fn index(&self) -> u32 {
        self.0
    }
}

impl Add for BytePos {
    type Output = BytePos;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub for BytePos {
    type Output = BytePos;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}
