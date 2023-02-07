use estrogen::{AsciiSprite, rgb};

use crate::world::{Entity, ObjectType};

use super::GRASS_BG;


impl Entity {
    pub fn get_tile_appearance(&self) -> AsciiSprite {
        match self {
            Self::Creature(_) => AsciiSprite { fg: rgb(1.0, 1.0, 0.0), bg: rgb(0.2, 0.1, 0.0), index: 'H' as _ },
            Self::Object(t) => match t {
                ObjectType::Stone => AsciiSprite { fg: rgb(0.6, 0.6, 0.6), bg: GRASS_BG, index: 'o' as _ },
            },
            Self::Me(_) => AsciiSprite { fg: rgb(1.0, 1.0, 0.0), bg: rgb(0.2, 0.1, 0.0), index: 1 },
        }
    }
}