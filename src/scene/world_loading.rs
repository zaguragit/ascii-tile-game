use std::sync::Mutex;

use estrogen::{AsciiSprite, RGB, rgb, UpdateResult, Loading};

use crate::world::World;

use super::world::create_world_scene;

#[repr(u8)]
#[derive(Debug)]
pub enum LoadingTask {
    GeneratingOverworld,
    GeneratingDungeons,
    PlacingPortals,
}

impl LoadingTask {
    fn get_label(&self) -> &str {
        match self {
            LoadingTask::GeneratingOverworld => "Generating overworld",
            LoadingTask::GeneratingDungeons => "Generating dungeons",
            LoadingTask::PlacingPortals => "Placing portals",
        }
    }
}

pub struct LoadingState {
    pub task: LoadingTask,
}

const WORLD_SIZE: usize = 256 * 3;
const LEVELS: usize = 2;

pub fn create_world_loading_scene() -> Loading<LoadingState, World<WORLD_SIZE, LEVELS>> {
    let mut state = LoadingState {
        task: LoadingTask::GeneratingOverworld,
    };
    Loading::new(state, get_char_at, load, on_loaded)
}

fn get_char_at(state: &LoadingState, x: usize, y: usize) -> AsciiSprite {
    const xoff: usize = 3;
    const yoff: usize = 3;
    const bg: RGB = rgb(0.1, 0.1, 0.1);
    const color: RGB = rgb(0.8, 0.5, 0.5);

    if y as i32 != yoff as i32 {
        return AsciiSprite { bg, fg: bg, index: 0 }
    }
    let option = state.task.get_label();

    let o = x as i32 - xoff as i32;
    if o < 0 || o as usize >= option.len() {
        return AsciiSprite { bg, fg: bg, index: 0 }
    }
    AsciiSprite { bg, fg: color, index: option.as_bytes()[o as usize] }
}

fn load(state: &Mutex<LoadingState>) -> World<WORLD_SIZE, LEVELS> {
    let seed = 5344545;
    let world = World::<WORLD_SIZE, LEVELS>::generate(seed);
    world
}

fn on_loaded(world: World<WORLD_SIZE, LEVELS>) -> UpdateResult {
    UpdateResult::SwitchScene(Box::new(create_world_scene(world)))
}