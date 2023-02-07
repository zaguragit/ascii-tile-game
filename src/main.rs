extern crate estrogen;

mod life;
mod player;
mod scene;
mod util;
mod world;

use scene::{main_menu::create_main_menu_scene, world_loading::create_world_loading_scene};

const TEXT_SIZE: (usize, usize) = (60, 46);

fn main() {
    let scene = Box::new(create_world_loading_scene());
    estrogen::game_loop("Uranium",
        TEXT_SIZE,
        ("terminal_8x8.png", 8),
        scene,
    );
}