use crate::types::Direction;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Lane {
    pub from: Direction,
    pub to: Direction,
}

impl Lane {
    pub fn set(from: Direction, to: Direction) -> Self {
        Lane { from, to }
    }
}
