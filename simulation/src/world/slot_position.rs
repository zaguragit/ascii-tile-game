

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SlotPosition {
    pub x: usize,
    pub y: usize,
    pub level: usize,
}

impl SlotPosition {
    pub fn to_level(&self, level: usize) -> SlotPosition {
        SlotPosition { level, ..*self }
    }
}