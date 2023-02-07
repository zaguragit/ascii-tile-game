extern crate glfw;
extern crate gl;

mod char_buffer;
mod key;
mod game_loop;
mod assets;
mod context;
mod scene;

pub use self::game_loop::{game_loop};
pub use self::{context::*, scene::*, key::*};
use glam::Vec3;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct AsciiSprite {
    pub fg: RGB,
    pub bg: RGB,
    pub index: u8,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RGB {
    pub const BLACK: RGB = rgb(0.0, 0.0, 0.0);

    pub fn to_argb(&self) -> u32 {
        0xff000000 | (((self.r * 255.0) as u32) << 16) | (((self.g * 255.0) as u32) << 8) | ((self.b * 255.0) as u32)
    }

    pub fn to_rgba(&self) -> u32 {
        (((self.r * 255.0) as u32) << 24) | (((self.g * 255.0) as u32) << 16) | (((self.b * 255.0) as u32) << 8) | 0xff
    }

    pub fn to_abgr(&self) -> u32 {
        0xff000000 | (((self.b * 255.0) as u32) << 16) | (((self.g * 255.0) as u32) << 8) | ((self.r * 255.0) as u32)
    }
}

#[inline(always)]
pub const fn rgb(r: f32, g: f32, b: f32) -> RGB { RGB { r, g, b } }

impl From<RGB> for Vec3 {
    fn from(rgb: RGB) -> Vec3 { Vec3 { x: rgb.r, y: rgb.g, z: rgb.b } }
}