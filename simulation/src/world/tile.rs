
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    GotoUnderworld,
    GotoOverworld,

    Void,

    OverworldGravel,
    OverworldScraps,
    OverworldWater,
    OverworldDeepWater,

    Stone,
    ThinGrass,
    Grass,

    // Trees
    AcaciaTree,
    BirchTree,
    GoldenTree,
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
        !matches!(self,
            Tile::Void | Tile::DungeonWall |
            Tile::DeepWater | Tile::Water |
            Tile::OverworldDeepWater | Tile::OverworldWater |
            Tile::AcaciaTree | Tile::BirchTree | Tile::GoldenTree | Tile::OakTree | Tile::PalmTree | Tile::PeachTree | Tile::SpruceTree)
    }
    pub fn is_swimmable(&self) -> bool {
        matches!(self, Tile::DeepWater | Tile::Water | 
            Tile::OverworldDeepWater | Tile::OverworldWater)
    }
    pub fn is_tree(&self) -> bool {
        matches!(self,
            Tile::PalmTree | Tile::SpruceTree |
            Tile::AcaciaTree | Tile::BirchTree | Tile::GoldenTree | Tile::OakTree | Tile::PeachTree)
    }
    pub fn is_grassy(&self) -> bool {
        matches!(self,
            Tile::Grass | Tile::ThinGrass |
            Tile::AcaciaTree | Tile::BirchTree | Tile::GoldenTree | Tile::OakTree | Tile::PeachTree)
    }
    pub fn is_stony(&self) -> bool {
        matches!(self, Tile::Stone | Tile::ThinGrass)
    }
    pub fn is_opaque(&self) -> bool {
        matches!(self, Tile::Void)
    }
}