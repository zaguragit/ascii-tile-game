extern crate engine;
extern crate simulation;

mod player;
mod scene;

use scene::{world_loading::create_world_loading_scene};

const TEXT_SIZE: (usize, usize) = (60, 46);

fn main() {
    let scene = Box::new(create_world_loading_scene());
    engine::game_loop("Uranium",
        TEXT_SIZE,
        ("assets/tileset.png", 8),
        scene,
    );
}