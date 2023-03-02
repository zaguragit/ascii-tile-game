mod game;
mod loading;
mod ui;

use glfw::Glfw;
use crate::{Context, AsciiSprite, Key};
pub use self::{game::*, loading::*, ui::*};

pub trait Scene {
    fn get_char_at(&self, x: usize, y: usize) -> AsciiSprite;
    fn on_loop(&mut self, context: &mut Context, glfw: &mut Glfw) -> UpdateResult;
    fn on_input(&mut self, key: Key, context: &mut Context, glfw: &mut Glfw) -> UpdateResult;
    fn on_attach(&mut self, context: &mut Context, glfw: &mut Glfw);
}

pub type CharFunction<State> = fn(&State, usize, usize) -> AsciiSprite;
pub type TickFunction<State> = fn(&mut State, &mut Context, Option<Key>) -> UpdateResult;
pub type InputFunction<State> = fn(&mut State, &mut Context, Key) -> UpdateResult;

pub enum UpdateResult {
    NoChange,
    Update,
    SwitchScene(Box<dyn Scene>),
    Quit,
}