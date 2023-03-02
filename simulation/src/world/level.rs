use std::ops::{Index, IndexMut};

use crate::util::FastRandom;

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

impl<const SIZE: usize> Level<SIZE> {
    pub fn swap_entities(&mut self, old: (usize, usize), new: (usize, usize)) {
        let from_old = self[old].entity;
        let from_new = self[new].entity;
        self[old].entity = from_new;
        self[new].entity = from_old;
    }

    pub fn find_floor(&self, random: &mut FastRandom) -> (usize, usize) {
        loop {
            let (x, y) = (random.next_less_than(SIZE as u64), random.next_less_than(SIZE as u64));
            let c = (x as usize, y as usize);
            let s = &self[c];
            if s.tile.is_floor() && s.entity.is_none() {
                return c;
            }
        }
    }
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