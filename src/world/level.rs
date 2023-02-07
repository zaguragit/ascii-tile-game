use std::ops::{Index, IndexMut};

use super::{Tile, Entity};

#[derive(Debug, Copy, Clone)]
pub struct Slot {
    pub tile: Tile,
    pub entity: Option<Entity>,
}

#[derive(Debug)]
pub struct Level<const SIZE: usize> {
    pub slots: Box<[Slot]>,
}

impl<const SIZE: usize> Index<(usize, usize)> for Level<SIZE> {
    type Output = Slot;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.slots[index.0 * SIZE + index.1]
    }
}

impl<const SIZE: usize> IndexMut<(usize, usize)> for Level<SIZE> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.slots[index.0 * SIZE + index.1]
    }
}