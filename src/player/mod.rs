use std::ops::{Index, IndexMut};

use crate::world::{World, SlotPosition, Tile};


pub struct Player<const SIZE: usize, const H: usize> {
    pub world: World<SIZE, H>,
}

impl<const SIZE: usize, const H: usize> Player<SIZE, H> {
    fn set_position(&mut self, position: SlotPosition) {
        let old_pos = self.world.player_position;
        let mut current_slot = self.world.mut_slot_at(&old_pos);
        let me = current_slot.entity.unwrap();
        current_slot.entity = None;
        let mut new_slot = self.world.mut_slot_at(&position);
        new_slot.entity = Some(me);
        self.world.player_position = position;
    }

    fn try_tp<'a>(&'a mut self, xoff: isize, yoff: isize) -> bool {
        let new_pos = self.world.position_relative_to_player(xoff, yoff);

        let next_slot = self.world.slot_at(&new_pos);
        if (next_slot.tile.is_floor() || next_slot.tile.is_swimmable()) && matches!(next_slot.entity, None) {
            self.set_position(new_pos);
            true
        } else { false }
    }

    pub fn try_enter(&mut self) -> bool {
        let old_pos = &self.world.player_position;
        match self.world[old_pos].tile {
            Tile::GotoOverworld => {
                self.set_position(old_pos.to_level(0));
                true
            },
            Tile::GotoDungeon => {
                self.set_position(old_pos.to_level(1));
                true
            },
            _ => false,
        }
    }

    pub fn step_up(&mut self) -> bool { self.try_tp(0, -1) }
    pub fn step_down(&mut self) -> bool { self.try_tp(0, 1) }
    pub fn step_left(&mut self) -> bool { self.try_tp(-1, 0) }
    pub fn step_right(&mut self) -> bool { self.try_tp(1, 0) }
}