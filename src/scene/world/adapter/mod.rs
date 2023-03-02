mod tile;

use engine::{rgb, RGB};

pub use self::tile::*;

const GRASS_BG: RGB = rgb(0.1, 0.45, 0.3);
const SAND_BG: RGB = rgb(1.0, 0.8, 0.6);
const SNOW_BG: RGB = rgb(0.9, 0.9, 0.9);