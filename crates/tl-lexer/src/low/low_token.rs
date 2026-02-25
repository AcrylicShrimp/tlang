use crate::low::LowTokenKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LowToken {
    pub kind: LowTokenKind,
    pub len: u32,
}
