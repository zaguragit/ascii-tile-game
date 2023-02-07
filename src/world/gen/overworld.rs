use crate::{world::{Level, Slot, Tile, Entity, ObjectType}, util::FastRandom};

use super::open_simplex_tileable_3d::OpenSimplexTileable3D;


pub(super) fn generate_overworld<const SIZE: usize>(seed: u64) -> Level<SIZE> {
    let mut level = Level {
        slots: vec![Slot {
            tile: Tile::DeepWater,
            entity: None,
        }; SIZE * SIZE].into_boxed_slice()
    };
    let s = SIZE as i32 / 6;
    let mut r = FastRandom::new(seed);
    let os64 = OpenSimplexTileable3D::new_with_seed_square(seed as i64, s / 64);
    let os32 = OpenSimplexTileable3D::new_with_seed_square(seed as i64 + 1, s / 32);
    let os16 = OpenSimplexTileable3D::new_with_seed_square(seed as i64 + 2, s / 16);
    let os8 = OpenSimplexTileable3D::new_with_seed_square(seed as i64 + 3, s / 8);
    let os4 = OpenSimplexTileable3D::new_with_seed_square(seed as i64 + 4, s / 4);
    for xi in 0..SIZE {
        for yi in 0..SIZE {
            let (x, y) = (xi as f64, yi as f64);
            let height_frequency = os64.eval(x / 64.0, y / 64.0, 192.0) * 0.5 + 0.5;
            let inv_height_frequency = 1.0 - height_frequency;
            let height =
                os64.eval(x / 64.0, y / 64.0, 0.0) * (0.8 * inv_height_frequency + 0.1 * height_frequency) +
                os32.eval(x / 32.0, y / 32.0, 0.0) * (0.1 * inv_height_frequency + 0.2 * height_frequency) +
                os16.eval(x / 16.0, y / 16.0, 0.0) * (0.05 * inv_height_frequency + 0.6 * height_frequency) +
                os8.eval(x / 8.0, y / 8.0, 0.0) * (0.05 * inv_height_frequency + 0.1 * height_frequency);
            let river = ((1.0 - os32.eval(x / 32.0, y / 32.0, 32.0).abs() * 5.0).powi(3) + os8.eval(x / 8.0, y / 8.0, 32.0) * 0.4).max(0.0);
            let river_zones = ((os32.eval(x / 32.0, y / 32.0, 48.0) * 0.5 + 0.5) * 20.0 - 10.0).clamp(0.0, 1.0);
            let river = river * river_zones;
            let h = (height - river.min(height.max(0.0))) + 0.1;
            let temperature = os64.eval(x / 64.0, y / 64.0, 64.0) * 0.6 +
                os32.eval(x / 32.0, y / 32.0, 64.0) * 0.3 +
                os4.eval(x / 4.0, y / 4.0, 64.0) * 0.1;
            let humidity =
                os64.eval(x / 64.0, y / 64.0, 128.0) * 0.7 +
                os16.eval(x / 16.0, y / 16.0, 128.0) * 0.2 +
                os8.eval(x / 8.0, y / 8.0, 128.0) * 0.1;
            let humidity = humidity * 0.8 + (1.0 - height.max(0.0)) * 0.2;
            level[(xi, yi)].tile = if h < -0.25 {
                Tile::DeepWater
            } else if h <= -0.0 {
                if temperature * (1.0 + h).powi(2) > 0.1 && r.next_less_than(4) == 0 {
                    Tile::SeaWeed
                } else {
                    Tile::Water
                }
            } else {
                if h < -0.2 + temperature * 0.4 {
                    if temperature > 0.3 && humidity > -0.1 && r.next_less_than(12) == 0 {
                        Tile::PalmTree
                    } else {
                        Tile::Sand
                    }
                } else {
                    if temperature < -0.2 {
                        if humidity > -0.2 && r.next_less_than(12) == 0 {
                            Tile::SpruceTree
                        } else {
                            Tile::Snow
                        }
                    } else if temperature * 0.8 - humidity * 0.2 > 0.1 {
                        Tile::Sand
                    } else if r.next_less_than(15) == 0 {
                        choose_tree_tile(temperature, humidity)
                    } else {
                        let volcanicness = os64.eval(x / 64.0, y / 64.0, 32.0) * 0.9 +
                            os4.eval(x / 4.0, y / 4.0, 32.0) * 0.1;
                        if volcanicness > 0.2 && r.next_less_than(24) == 0 {
                            level[(xi, yi)].entity = Some(Entity::Object(ObjectType::Stone))
                        }
                        if humidity < -0.5 {
                            Tile::Dirt
                        } else if humidity < -0.4 {
                            Tile::ThinGrass
                        } else {
                            Tile::Grass
                        }
                    }
                }
            };
        }
    }
    level
}

fn choose_tree_tile(temperature: f64, humidity: f64) -> Tile {
    if temperature > 0.5 {
        Tile::AcaciaTree
    } else if temperature > -0.1 {
        if humidity > 0.5 {
            Tile::PeachTree
        } else {
            Tile::OakTree
        }
    } else {
        Tile::BirchTree
    }
}