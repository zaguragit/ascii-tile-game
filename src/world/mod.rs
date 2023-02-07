mod entity;
mod gen;
mod level;
mod tile;

use std::{ops::{Index, IndexMut}, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

pub use self::{tile::*, entity::*, level::*};

pub struct World<const SIZE: usize, const H: usize> {
    levels: [Level<SIZE>; H],
    pub player_position: SlotPosition,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct SlotPosition {
    x: usize,
    y: usize,
    level: usize,
}

impl SlotPosition {
    pub fn to_level(&self, level: usize) -> SlotPosition {
        SlotPosition { level, ..*self }
    }
    pub fn uid(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl<const SIZE: usize, const H: usize> World<SIZE, H> {
    pub fn position_relative_to_player(&self, x: isize, y: isize) -> SlotPosition {
        let x = (self.player_position.x as isize + x as isize + SIZE as isize) as usize % SIZE;
        let y = (self.player_position.y as isize + y as isize + SIZE as isize) as usize % SIZE;
        SlotPosition { x, y, level: self.player_position.level }
    }
    pub fn slot_at<'a>(&'a self, position: &SlotPosition) -> &'a Slot {
        &self.levels[position.level][(position.x, position.y)]
    }
    pub fn mut_slot_at<'a>(&'a mut self, position: &SlotPosition) -> &'a mut Slot {
        &mut self.levels[position.level][(position.x, position.y)]
    }
}

impl<const SIZE: usize, const H: usize> Index<&SlotPosition> for World<SIZE, H> {
    type Output = Slot;

    fn index(&self, index: &SlotPosition) -> &Self::Output {
        &self.levels[index.level][(index.x, index.y)]
    }
}

impl<const SIZE: usize, const H: usize> IndexMut<&SlotPosition> for World<SIZE, H> {
    fn index_mut(&mut self, index: &SlotPosition) -> &mut Self::Output {
        &mut self.levels[index.level][(index.x, index.y)]
    }
}

impl<const SIZE: usize, const H: usize> Index<usize> for World<SIZE, H> {
    type Output = Level<SIZE>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.levels[index]
    }
}

impl<const SIZE: usize, const H: usize> IndexMut<usize> for World<SIZE, H> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.levels[index]
    }
}