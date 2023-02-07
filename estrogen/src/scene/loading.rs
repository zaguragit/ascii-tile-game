
use std::{thread::{self, JoinHandle}, sync::{Arc, Mutex}};
use glfw::Glfw;

use crate::{CharFunction, AsciiSprite, Scene, Context, Key, UpdateResult};

pub type LoadingFunction<State, LoadedData> = fn(&Mutex<State>) -> LoadedData;
pub type OnLoadedFunction<LoadedData> = fn(LoadedData) -> UpdateResult;

pub struct Loading<State, LoadedData> {
    state: Arc<Mutex<State>>,
    thread: Option<JoinHandle<LoadedData>>,
    get_char_at: CharFunction<State>,
    load: LoadingFunction<State, LoadedData>,
    on_loaded: OnLoadedFunction<LoadedData>,
}

impl<State, LoadedData> Loading<State, LoadedData> {
    pub fn new(
        state: State,
        get_char_at: CharFunction<State>,
        load: LoadingFunction<State, LoadedData>,
        on_loaded: OnLoadedFunction<LoadedData>,
    ) -> Self {
        Self { state: Arc::new(Mutex::new(state)), get_char_at, load, on_loaded, thread: None }
    }
}

impl<State: Send + 'static, LoadedData: Send + 'static> Scene for Loading<State, LoadedData> {
    fn get_char_at(&self, x: usize, y: usize) -> AsciiSprite {
        let l = self.state.as_ref().lock().unwrap();
        (self.get_char_at)(&*l, x, y)
    }
    fn on_loop(&mut self, _context: &mut Context, _glfw: &mut Glfw) -> UpdateResult {
        if self.thread.as_ref().unwrap().is_finished() {
            (self.on_loaded)(self.thread.take().unwrap().join().expect("Loading thread failed"))
        } else {
            UpdateResult::NoChange
        }
    }
    fn on_input(&mut self, _key: Key, _context: &mut Context, _glfw: &mut Glfw) -> UpdateResult { UpdateResult::NoChange }
    fn on_attach(&mut self, _context: &mut Context, _glfw: &mut Glfw) {
        let load = self.load;
        let state = Arc::clone(&self.state);
        self.thread = Some(thread::spawn(move || load(state.as_ref())));
    }
}