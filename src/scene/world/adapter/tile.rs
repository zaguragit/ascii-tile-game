
use estrogen::{AsciiSprite, rgb, RGB};

use crate::world::Tile;

use super::{GRASS_BG, SAND_BG, SNOW_BG};

impl Tile {
    pub fn get_tile_appearance(&self) -> AsciiSprite {
        match self {
            Tile::GotoDungeon    => AsciiSprite { fg: rgb(1.0, 1.0, 1.0), bg: RGB::BLACK, index: '%' as _ },
            Tile::GotoOverworld  => AsciiSprite { fg: rgb(1.0, 1.0, 1.0), bg: RGB::BLACK, index: '%' as _ },

            Tile::Void           => AsciiSprite { fg: RGB::BLACK, bg: RGB::BLACK, index: 0 },
            
            Tile::Dirt           => AsciiSprite { fg: rgb(0.7, 0.4, 0.35), bg: rgb(0.4, 0.3, 0.2), index: '.' as _ },
            Tile::Sand           => AsciiSprite { fg: rgb(0.7, 0.5, 0.3), bg: SAND_BG, index: '.' as _ },
            Tile::Snow           => AsciiSprite { fg: rgb(1.0, 1.0, 1.0), bg: SNOW_BG, index: 176 },
            
            Tile::ThinGrass      => AsciiSprite { fg: rgb(0.25, 0.5, 0.2), bg: rgb(0.4, 0.3, 0.2), index: '"' as _ },
            Tile::Grass          => AsciiSprite { fg: rgb(0.25, 0.6, 0.2), bg: GRASS_BG, index: '"' as _ },

            Tile::AcaciaTree     => AsciiSprite { fg: rgb(0.8, 0.9, 0.1), bg: GRASS_BG, index: 5 },
            Tile::BirchTree      => AsciiSprite { fg: rgb(0.7, 0.8, 0.7), bg: GRASS_BG, index: 'Y' as _ },
            Tile::OakTree        => AsciiSprite { fg: rgb(0.4, 0.8, 0.36), bg: GRASS_BG, index: 5 },
            Tile::PeachTree      => AsciiSprite { fg: rgb(1.0, 0.6, 0.7), bg: GRASS_BG, index: 6 },
            
            Tile::PalmTree       => AsciiSprite { fg: rgb(0.1, 0.4, 0.2), bg: SAND_BG, index: 'T' as _ },
            Tile::SpruceTree     => AsciiSprite { fg: rgb(0.1, 0.4, 0.4), bg: SNOW_BG, index: 6 },
            
            Tile::SeaWeed        => AsciiSprite { fg: rgb(0.2, 0.7, 0.4), bg: rgb(0.1, 0.36, 0.5), index: '"' as _ },
            Tile::Water          => AsciiSprite { fg: rgb(0.2, 0.6, 0.8), bg: rgb(0.1, 0.36, 0.5), index: '~' as _ },
            Tile::DeepWater      => AsciiSprite { fg: rgb(0.2, 0.6, 0.8), bg: rgb(0.02, 0.3, 0.5), index: '~' as _ },

            Tile::DungeonFloor   => AsciiSprite { fg: rgb(0.5, 0.5, 0.5), bg: RGB::BLACK, index: '.' as _ },
            Tile::DungeonWall    => AsciiSprite { fg: rgb(0.8, 0.8, 0.8), bg: RGB::BLACK, index: '#' as _ },
        }
    }
}