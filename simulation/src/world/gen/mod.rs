mod open_simplex_tileable_3d;
mod underworld;
mod overworld;

use crate::util::FastRandom;
use self::{overworld::generate_overworld, underworld::generate_underworld};

use super::{Tile, World, Entity, SlotPosition, Level, creature::{Creature, creature_from_species, SpeciesID, SpeciesMap, base_creature_from_species}, Ambient};

impl<const SIZE: usize> World<SIZE, 2> {
    pub fn generate(seed: u64) -> Self {
        let mut overworld = generate_overworld(seed);
        let (mut underworld, rooms) = generate_underworld(seed, 32);
        let mut random = FastRandom::new(seed - 1);
        place_portals(&mut random, &mut overworld, &mut underworld);
        Self {
            levels: [overworld, underworld],
            player_position: SlotPosition { x: 0, y: 0, level: 0 },
            species: SpeciesMap::new(),
        }
    }

    pub fn create_life(&mut self, seed: u64, species: SpeciesMap, player_species: SpeciesID) {
        let mut random = FastRandom::new(seed - 2);
        let sparseness = 16;
        for level in &mut self.levels {
            for _ in 0..(SIZE / sparseness * SIZE / sparseness) {
                let f = level.find_floor(&mut random);
                let ambient = level.get_ambient_at(f.0, f.1, 12);
                let mut s = &mut level[f];
                s.entity = Some(Entity::Creature(generate_creature(&species, &mut random, &ambient)));
            }
        }
        let player_xy = self.levels[1].find_floor(&mut random);
        let player_position = SlotPosition { x: player_xy.0, y: player_xy.1, level: 1 };
        self[&player_position].entity = Some(Entity::Creature(
            base_creature_from_species(&species, player_species)
        ));
        self.player_position = player_position;
        self.species = species;
    }
}

fn place_portals<const SIZE: usize>(random: &mut FastRandom, overworld: &mut Level<SIZE>, dungeon: &mut Level<SIZE>) {
    for _ in 0..(SIZE / 32 * SIZE / 32) {
        let (x, y) = (random.next_less_than(SIZE as u64), random.next_less_than(SIZE as u64));
        let mut overworld_slot = &mut overworld[(x as usize, y as usize)];
        let mut dungeon_slot = &mut dungeon[(x as usize, y as usize)];
        if overworld_slot.tile.is_floor() && dungeon_slot.tile.is_floor() {
            overworld_slot.tile = Tile::GotoUnderworld;
            dungeon_slot.tile = Tile::GotoOverworld;
        }
    }
}

fn generate_creature(species: &SpeciesMap, random: &mut FastRandom, ambient: &Ambient) -> Creature {
    creature_from_species(random, species, SpeciesID(0))
}

fn carve_from<const SIZE: usize>(
    level: &mut Level<SIZE>,
    random: &mut FastRandom,
    max_distance: usize,
    mut from_x: usize,
    mut from_y: usize,
    to_x: usize,
    to_y: usize,
    tile: Tile,
    only_on_void: bool,
) {
    let length = ((from_x.abs_diff(to_x).pow(2) + from_y.abs_diff(to_y).pow(2)) as f64).sqrt() as usize;
    if length >= max_distance {
        return;
    }
    let radius_multiplier = random.next_less_than(256) as f64 / 255.0;
    for i in (-(length as isize) / 3)..=length as isize {
        let ratio = i as f64 / length as f64;
        let inv_ratio = 1.0 - ratio;
        let x = to_x as f64 * inv_ratio + from_x as f64 * ratio;
        let y = to_y as f64 * inv_ratio + from_y as f64 * ratio;
        let radius = 1.5 + radius_multiplier * 2.2 + inv_ratio * inv_ratio * 1.2;
        let sr = radius * radius;
        for bx in (x - radius) as i64 ..=(x + radius) as i64 {
            for by in (y - radius) as i64 ..=(y + radius) as i64 {
                if (bx as f64 - x) * (bx as f64 - x) + (by as f64 - y) * (by as f64 - y) > sr { continue; }
                let x = (bx + SIZE as i64) as usize % SIZE;
                let y = (by + SIZE as i64) as usize % SIZE;
                let c = (x, y);
                if !only_on_void || matches!(level[c].tile, Tile::Void) {
                    level[c].tile = tile;
                }
            }
        }
        if random.one_in((1.5 + inv_ratio * 6.2) as u64) {
            from_x = (from_x as i64 + random.next_less_than(9) as i64 - 4 + SIZE as i64) as usize % SIZE;
            from_y = (from_y as i64 + random.next_less_than(9) as i64 - 4 + SIZE as i64) as usize % SIZE;
        }
    }
}