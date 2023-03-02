use crate::{world::{Level, Slot, Tile}, util::FastRandom};

use super::{open_simplex_tileable_3d::OpenSimplexTileable3D, carve_from};

pub(super) fn generate_overworld<const SIZE: usize>(seed: u64) -> Level<SIZE> {
    let mut level = Level {
        slots: vec![Slot {
            tile: Tile::OverworldDeepWater,
            entity: None,
        }; SIZE * SIZE].into_boxed_slice()
    };
    let mut random = FastRandom::new(seed);
    place_terrain(&mut level, &mut random);
    carve_rivers(&mut level, &mut random);
    level
}

fn place_terrain<const SIZE: usize>(
    level: &mut Level<SIZE>,
    random: &mut FastRandom,
) {
    let s = SIZE as i32 / 6;
    let os64 = OpenSimplexTileable3D::new_with_seed_square(random.next() as i64, s / 64);
    let os16 = OpenSimplexTileable3D::new_with_seed_square(random.next() as i64, s / 16);
    let os8 = OpenSimplexTileable3D::new_with_seed_square(random.next() as i64, s / 8);
    for xi in 0..SIZE {
        for yi in 0..SIZE {
            const DEEP_WATER_LEVEL: f64 = -0.1;
            const TEMPERATURE_BEACH_WEIGHT: f64 = 0.4;
            let (x, y) = (xi as f64, yi as f64);
            let height_frequency = (os64.eval(x / 64.0, y / 64.0, 192.0) * 0.5 + 0.5).powi(2);
            let inv_height_frequency = 1.0 - height_frequency;
            let weights = [
                1.0 * inv_height_frequency + 0.0 * height_frequency,
                0.0 * inv_height_frequency + 0.6 * height_frequency,
                0.0 * inv_height_frequency + 0.4 * height_frequency,
            ];
            let height = 0.1 + os64.eval(x / 64.0, y / 64.0, 0.0) * weights[0];
            let height = if
                height + weights[1] + weights[2] < DEEP_WATER_LEVEL ||
                height - weights[1] - weights[2] > TEMPERATURE_BEACH_WEIGHT {
                    height
                } else { height + os16.eval(x / 16.0, y / 16.0, 0.0) * weights[1] };
            let height = if
                height + weights[2] < DEEP_WATER_LEVEL ||
                height - weights[2] > TEMPERATURE_BEACH_WEIGHT {
                    height
                } else { height + os8.eval(x / 8.0, y / 8.0, 0.0) * weights[2] };

            level[(xi, yi)].tile = if height < DEEP_WATER_LEVEL {
                Tile::OverworldDeepWater
            } else if height <= -0.0 {
                Tile::OverworldWater
            } else if random.one_in(12) {
                Tile::OverworldScraps
            } else {
                Tile::OverworldGravel
            };
        }
    }
}

fn carve_rivers<const SIZE: usize>(
    level: &mut Level<SIZE>,
    random: &mut FastRandom,
) {
    let river_sparceness = 32;
    for _ in 0..(SIZE / river_sparceness * SIZE / river_sparceness) {
        let (from_x, from_y) = level.find_floor(random);
        let max_distance = 192;
        if let Some((to_x, to_y)) = closest_water_tile(random, level, from_x, from_y, max_distance) {
            carve_from(level, random, max_distance, from_x, from_y, to_x, to_y, Tile::OverworldWater, false);
        }
    }
}

fn closest_water_tile<const SIZE: usize>(
    random: &mut FastRandom,
    level: &Level<SIZE>,
    x: usize,
    y: usize,
    max_axis_distance: usize,
) -> Option<(usize, usize)> {
    let mut dist = 1;
    'l: loop {
        if random.one_in(2) {
            continue;
        }
        for x in (x as i64 - dist)..=(x as i64 + dist) {
            let c = ((x + SIZE as i64) as usize % SIZE, (y as i64 - dist + SIZE as i64) as usize % SIZE);
            if matches!(level[c].tile, Tile::OverworldWater | Tile::OverworldDeepWater) {
                break 'l Some(c);
            }
            let c = ((x + SIZE as i64) as usize % SIZE, (y as i64 + dist) as usize % SIZE);
            if matches!(level[c].tile, Tile::OverworldWater | Tile::OverworldDeepWater) {
                break 'l Some(c);
            }
        }
        for y in (y as i64 - dist + 1)..(y as i64 + dist) {
            let c = ((x as i64 - dist + SIZE as i64) as usize % SIZE, (y + SIZE as i64) as usize % SIZE);
            if matches!(level[c].tile, Tile::OverworldWater | Tile::OverworldDeepWater) {
                break 'l Some(c);
            }
            let c = ((x as i64 + dist) as usize % SIZE, (y + SIZE as i64) as usize % SIZE);
            if matches!(level[c].tile, Tile::OverworldWater | Tile::OverworldDeepWater) {
                break 'l Some(c);
            }
        }
        if dist as usize >= max_axis_distance {
            break 'l None;
        }
        dist += 1;
    }
}