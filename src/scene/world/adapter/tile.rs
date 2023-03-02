
use engine::{AsciiSprite, rgb, RGB};

use simulation::{world::Tile, util::FastRandom};

use super::{GRASS_BG, SAND_BG, SNOW_BG};

pub fn tile_to_ascii_sprite(tile: &Tile, random: &mut FastRandom) -> AsciiSprite {
    match tile {
        Tile::GotoUnderworld => AsciiSprite { fg: RGB::WHITE, bg: RGB::BLACK, index: '%' as _ },
        Tile::GotoOverworld  => AsciiSprite { fg: RGB::WHITE, bg: RGB::BLACK, index: '%' as _ },

        Tile::OverworldGravel => AsciiSprite { fg: rgb(0.55, 0.55, 0.5), bg: rgb(0.45, 0.45, 0.4), index: ['.', '.', '.', ' ', ' ', '_', '^'][random.next_less_than(7) as usize] as _ },
        Tile::OverworldScraps => AsciiSprite { fg: rgb(0.5, 0.5, 0.5), bg: rgb(0.4, 0.4, 0.4), index: ['"', '\'', '`', '"'][random.next_less_than(4) as usize] as _ },
        Tile::OverworldWater  => AsciiSprite { fg: rgb(0.55, 0.5, 0.25), bg: rgb(0.15, 0.22, 0.15), index: ['~', ' ', ' '][random.next_less_than(3) as usize] as _ },
        Tile::OverworldDeepWater => AsciiSprite { fg: rgb(0.45, 0.5, 0.2), bg: rgb(0.2, 0.18, 0.0), index: ['~', '~', '-', ' ', ' '][random.next_less_than(5) as usize] as _ },

        Tile::Void           => AsciiSprite { fg: RGB::BLACK, bg: RGB::BLACK, index: 0 },
        
        Tile::Stone          => AsciiSprite { fg: rgb(0.5, 0.5, 0.5), bg: rgb(0.3, 0.3, 0.3), index: ['.', '.', '.', ',', ',', '_'][random.next_less_than(6) as usize] as _ },
        Tile::ThinGrass      => AsciiSprite { fg: rgb(0.2, 0.45, 0.2), bg: rgb(0.28, 0.36, 0.31), index: ['"', '\'', '`', '"'][random.next_less_than(4) as usize] as _ },
        Tile::Grass          => AsciiSprite { fg: rgb(0.2, 0.5, 0.3), bg: GRASS_BG, index: ['"', '\'', '`', '"'][random.next_less_than(4) as usize] as _ },

        Tile::AcaciaTree     => AsciiSprite { fg: rgb(0.8, 0.9, 0.1), bg: GRASS_BG, index: 5 },
        Tile::BirchTree      => AsciiSprite { fg: rgb(0.7, 0.8, 0.7), bg: GRASS_BG, index: 'Y' as _ },
        Tile::GoldenTree     => AsciiSprite { fg: rgb(0.9, 0.8, 0.1), bg: GRASS_BG, index: 5 },
        Tile::OakTree        => AsciiSprite { fg: rgb(0.4, 0.8, 0.36), bg: GRASS_BG, index: 5 },
        Tile::PeachTree      => AsciiSprite { fg: rgb(1.0, 0.6, 0.7), bg: GRASS_BG, index: 6 },
        
        Tile::PalmTree       => AsciiSprite { fg: rgb(0.1, 0.4, 0.2), bg: SAND_BG, index: 'T' as _ },
        Tile::SpruceTree     => AsciiSprite { fg: rgb(0.1, 0.4, 0.4), bg: SNOW_BG, index: 6 },
        
        Tile::SeaWeed        => AsciiSprite { fg: rgb(0.3, 0.7, 0.4), bg: rgb(0.1, 0.4, 0.7), index: '"' as _ },
        Tile::Water          => AsciiSprite { fg: rgb(0.25, 0.55, 0.6), bg: rgb(0.1, 0.4, 0.7), index: ['~', ' ', ' '][random.next_less_than(3) as usize] as _ },
        Tile::DeepWater      => AsciiSprite { fg: rgb(0.2, 0.45, 0.6), bg: rgb(0.1, 0.35, 0.6), index: ['~', '~', '-', ' ', ' '][random.next_less_than(5) as usize] as _ },
        
        Tile::DungeonFloor   => AsciiSprite { fg: rgb(0.5, 0.5, 0.5), bg: RGB::BLACK, index: ['.', '.', '.', ',', ',', '_'][random.next_less_than(6) as usize] as _ },
        Tile::DungeonWall    => AsciiSprite { fg: rgb(0.8, 0.8, 0.8), bg: RGB::BLACK, index: '#' as _ },
    }
}