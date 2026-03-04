use crate::BytePos;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
}

impl Span {
    pub const fn new(lo: BytePos, hi: BytePos) -> Self {
        Self { lo, hi }
    }

    pub fn is_valid(self) -> bool {
        self.lo <= self.hi
    }

    pub fn is_empty(self) -> bool {
        self.lo >= self.hi
    }

    pub fn contains_pos(self, pos: BytePos) -> bool {
        self.lo <= pos && self.hi >= pos
    }

    pub fn contains_span(self, rhs: Self) -> bool {
        self.lo <= rhs.lo && self.hi >= rhs.hi
    }

    pub fn intersects_span(self, rhs: Self) -> bool {
        self.lo <= rhs.hi && self.hi >= rhs.lo
    }

    pub fn len(self) -> u32 {
        self.hi
            .index()
            .checked_sub(self.lo.index())
            .unwrap_or_default()
    }

    pub fn union(lhs: Self, rhs: Self) -> Self {
        Self::new(lhs.lo.min(rhs.lo), lhs.hi.max(rhs.hi))
    }

    pub fn intersection(lhs: Self, rhs: Self) -> Self {
        Self::new(lhs.lo.max(rhs.lo), lhs.hi.min(rhs.hi))
    }
}
