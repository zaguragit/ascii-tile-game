use glfw::Glfw;

use crate::{CharFunction, InputFunction, AsciiSprite, Scene, Context, Key, UpdateResult};

pub mod util;

pub struct UI<State> {
    state: State,
    get_char_at: CharFunction<State>,
    on_input: InputFunction<State>,
}

impl<State> UI<State> {
    pub fn new(
        state: State,
        get_char_at: CharFunction<State>,
        on_input: InputFunction<State>,
    ) -> Self {
        Self { state, get_char_at, on_input }
    }
}

impl<State> Scene for UI<State> {
    fn get_char_at(&self, x: usize, y: usize) -> AsciiSprite {
        (self.get_char_at)(&self.state, x, y)
    }
    fn on_loop(&mut self, _context: &mut Context, _glfw: &mut Glfw) -> UpdateResult { UpdateResult::NoChange }
    fn on_input(&mut self, key: Key, context: &mut Context, _glfw: &mut Glfw) -> UpdateResult {
        (self.on_input)(&mut self.state, context, key)
    }
    fn on_attach(&mut self, _context: &mut Context, _glfw: &mut Glfw) {}
}