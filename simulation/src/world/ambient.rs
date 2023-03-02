use std::fmt::Display;

use super::{World, SlotPosition, Level};

pub enum Biome {
    GrassyArea(WaterCloseness),
    Forest(WaterCloseness),
    PlainCave(WaterCloseness),
    Sea,
}

pub enum WaterCloseness {
    Land,
    Coast,
}

impl Display for Biome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Biome::GrassyArea(w) => write!(f, "grassy area, {}", w),
            Biome::Forest(w) => write!(f, "forest, {}", w),
            Biome::PlainCave(w) => write!(f, "plain cave, {}", w),
            Biome::Sea => write!(f, "sea"),
        }
    }
}

impl Display for WaterCloseness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WaterCloseness::Land => "land",
            WaterCloseness::Coast => "coast",
        })
    }
}

pub struct Ambient {
    pub biome: Biome,
}

impl Display for Ambient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.biome)
    }
}

impl<const SIZE: usize, const H: usize> World<SIZE, H> {
    pub fn get_ambient_at(
        &self,
        at: &SlotPosition,
        radius: usize,
    ) -> Ambient {
        self.levels[at.level].get_ambient_at(at.x, at.y, radius)
    }
}

impl<const SIZE: usize> Level<SIZE> {
    pub fn get_ambient_at(
        &self,
        x: usize,
        y: usize,
        radius: usize,
    ) -> Ambient {
        let total_tiles = (radius * 2 + 1) * (radius * 2 + 1);
        let mut swimmable_count = 0;
        let mut grassy_count = 0;
        let mut stony_count = 0;
        let mut tree_count = 0;
        for x in (x as i64 - radius as i64)..=(x + radius) as i64 {
            for y in (y as i64 - radius as i64)..=(y + radius) as i64 {
                let slot = &self[((x + SIZE as i64) as usize % SIZE, (y + SIZE as i64) as usize % SIZE)];
                if slot.tile.is_tree() {
                    tree_count += 1;
                }
                if slot.tile.is_grassy() {
                    grassy_count += 1;
                }
                else if slot.tile.is_stony() {
                    stony_count += 1;
                }
                else if slot.tile.is_swimmable() {
                    swimmable_count += 1;
                }
            }
        }
        let biome = if swimmable_count > grassy_count && swimmable_count > stony_count {
            Biome::Sea
        } else {
            let swimmable_ratio = swimmable_count as f64 / total_tiles as f64;
            let tree_ratio = tree_count as f64 / total_tiles as f64;
            let w = if swimmable_ratio > 0.3 {
                WaterCloseness::Coast
            } else {
                WaterCloseness::Land
            };
            if grassy_count > stony_count {
                if tree_ratio > 0.1 { Biome::Forest(w) }
                else { Biome::GrassyArea(w) }
            } else {
                Biome::PlainCave(w)
            }
        };
        Ambient {
            biome,
        }
    }
}