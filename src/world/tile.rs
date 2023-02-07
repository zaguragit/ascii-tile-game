
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    GotoDungeon,
    GotoOverworld,

    Void,

    Dirt,
    Sand,
    Snow,

    ThinGrass,
    Grass,

    // Trees
    AcaciaTree,
    BirchTree,
    OakTree,
    PeachTree,

    PalmTree,
    SpruceTree,
    
    DungeonFloor,
    DungeonWall,

    // Water
    SeaWeed,
    Water,
    DeepWater,
}

impl Tile {
    pub fn is_floor(&self) -> bool {
        match self {
            Tile::Void | Tile::DungeonWall |
            Tile::DeepWater | Tile::Water |
            Tile::AcaciaTree | Tile::BirchTree | Tile::OakTree | Tile::PalmTree | Tile::PeachTree | Tile::SpruceTree => false,
            _ => true,
        }
    }
    pub fn is_swimmable(&self) -> bool {
        match self {
            Tile::DeepWater | Tile::Water => true,
            _ => false,
        }
    }
}