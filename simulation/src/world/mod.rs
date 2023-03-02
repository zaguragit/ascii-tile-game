mod ambient;
mod entity;
mod gen;
mod level;
mod slot_position;
mod tile;

use std::ops::{Index, IndexMut};

use self::creature::SpeciesMap;
pub use self::{ambient::*, tile::*, entity::*, level::*, slot_position::*};

pub struct World<const SIZE: usize, const H: usize> {
    pub levels: [Level<SIZE>; H],
    pub player_position: SlotPosition,
    pub species: SpeciesMap,
}

impl<const SIZE: usize, const H: usize> World<SIZE, H> {
    pub fn position_relative_to_player(&self, x: isize, y: isize) -> SlotPosition {
        let x = (self.player_position.x as isize + x as isize + SIZE as isize) as usize % SIZE;
        let y = (self.player_position.y as isize + y as isize + SIZE as isize) as usize % SIZE;
        SlotPosition { x, y, level: self.player_position.level }
    }
    pub fn swap_entities(&mut self, old: SlotPosition, new: SlotPosition) {
        let from_old = self[&old].entity;
        let from_new = self[&new].entity;
        self[&old].entity = from_new;
        self[&new].entity = from_old;
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