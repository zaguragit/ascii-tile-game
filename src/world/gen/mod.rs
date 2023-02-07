mod open_simplex_tileable_3d;
mod dungeon;
mod overworld;

use crate::{util::FastRandom, player};
use self::{overworld::generate_overworld, dungeon::generate_dungeon};

use super::{Tile, World, Entity, CreatureStats, SlotPosition, Level};

impl<const SIZE: usize> World<SIZE, 2> {
    pub fn generate(seed: u64) -> Self {
        let mut overworld = generate_overworld(seed);
        let mut dungeon = generate_dungeon(seed);
        let mut random = FastRandom::new(seed - 1);
        place_portals(&mut random, &mut overworld, &mut dungeon);
        let player_xy = find_floor(&mut random, &mut overworld);
        overworld[player_xy].entity = Some(make_player());
        Self {
            levels: [overworld, dungeon],
            player_position: SlotPosition { x: player_xy.0, y: player_xy.1, level: 0 },
        }
    }
}

fn place_portals<const SIZE: usize>(random: &mut FastRandom, overworld: &mut Level<SIZE>, dungeon: &mut Level<SIZE>) {
    for _ in 0..(SIZE / 32 * SIZE / 32) {
        let (x, y) = (random.next_less_than(SIZE as u64), random.next_less_than(SIZE as u64));
        let mut overworld_slot = &mut overworld[(x as usize, y as usize)];
        let mut dungeon_slot = &mut dungeon[(x as usize, y as usize)];
        if overworld_slot.tile.is_floor() && dungeon_slot.tile.is_floor() {
            overworld_slot.tile = Tile::GotoDungeon;
            dungeon_slot.tile = Tile::GotoOverworld;
        }
    }
}

fn find_floor<const SIZE: usize>(random: &mut FastRandom, level: &mut Level<SIZE>) -> (usize, usize) {
    let mut pos = (0, 0);
    loop {
        let (x, y) = (random.next_less_than(SIZE as u64), random.next_less_than(SIZE as u64));
        let c = (x as usize, y as usize);
        if level[c].tile.is_floor() {
            return c;
        }
    }
}

fn make_player() -> Entity {
    Entity::Me(CreatureStats {
        health: 255,
        strength: 12,
        intimidation: 12,
        karma: 0,
    })
}