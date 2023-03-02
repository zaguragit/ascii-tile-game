extern crate glfw;
extern crate gl;

mod char_buffer;
mod key;
mod game_loop;
mod assets;
mod context;
mod scene;

use std::ops::Mul;

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
    pub const WHITE: RGB = rgb(1.0, 1.0, 1.0);

    pub fn to_argb(&self) -> u32 {
        0xff000000 | (((self.r * 255.0) as u32) << 16) | (((self.g * 255.0) as u32) << 8) | ((self.b * 255.0) as u32)
    }

    pub fn to_rgba(&self) -> u32 {
        (((self.r * 255.0) as u32) << 24) | (((self.g * 255.0) as u32) << 16) | (((self.b * 255.0) as u32) << 8) | 0xff
    }

    pub fn to_abgr(&self) -> u32 {
        0xff000000 | (((self.b * 255.0) as u32) << 16) | (((self.g * 255.0) as u32) << 8) | ((self.r * 255.0) as u32)
    }

    pub fn squared_perceived_lightness(&self) -> f32 {
        0.299 * self.r * self.r +
        0.587 * self.g * self.g +
        0.114 * self.b * self.b
    }
}

#[inline(always)]
pub const fn rgb(r: f32, g: f32, b: f32) -> RGB { RGB { r, g, b } }

#[inline(always)]
pub const fn rgb_gray(l: f32) -> RGB { RGB { r: l, g: l, b: l } }

impl Mul for RGB {
    type Output = Self;
    fn mul(self, o: Self) -> Self::Output {
        rgb(self.r * o.r, self.g * o.g, self.b * o.b)
    }
}

impl From<RGB> for Vec3 {
    fn from(rgb: RGB) -> Vec3 { Vec3 { x: rgb.r, y: rgb.g, z: rgb.b } }
}