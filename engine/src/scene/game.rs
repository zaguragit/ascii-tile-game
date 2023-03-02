
use glfw::Glfw;

use crate::{CharFunction, AsciiSprite, Scene, Context, Key, UpdateResult, TickFunction};

pub struct Game<State> {
    state: State,
    get_char_at: CharFunction<State>,
    on_tick: TickFunction<State>,
    seconds_per_passive_tick: f64,
    last_tick: f64,
}

impl<State> Game<State> {
    pub fn new(
        state: State,
        get_char_at: CharFunction<State>,
        on_tick: TickFunction<State>,
        seconds_per_passive_tick: f64,
    ) -> Self {
        Self { state, get_char_at, on_tick, seconds_per_passive_tick, last_tick: 0.0 }
    }
}

impl<State> Scene for Game<State> {
    fn get_char_at(&self, x: usize, y: usize) -> AsciiSprite {
        (self.get_char_at)(&self.state, x, y)
    }
    fn on_loop(&mut self, context: &mut Context, glfw: &mut Glfw) -> UpdateResult {
        let end = glfw.get_time();
        if end - self.last_tick > self.seconds_per_passive_tick {
            self.last_tick = end;
            (self.on_tick)(&mut self.state, context, None)
        } else { UpdateResult::NoChange }
    }
    fn on_input(&mut self, key: Key, context: &mut Context, glfw: &mut Glfw) -> UpdateResult {
        self.last_tick = glfw.get_time();
        (self.on_tick)(&mut self.state, context, Some(key))
    }
    fn on_attach(&mut self, context: &mut Context, glfw: &mut Glfw) {
        self.last_tick = glfw.get_time();
    }
}