mod entity;
mod tile;

use estrogen::{rgb, RGB};

pub use self::{tile::*, entity::*};

const GRASS_BG: RGB = rgb(0.1, 0.4, 0.3);
const SAND_BG: RGB = rgb(1.0, 0.7, 0.5);
const SNOW_BG: RGB = rgb(0.9, 0.9, 0.9);