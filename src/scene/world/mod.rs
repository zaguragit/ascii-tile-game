use estrogen::{AsciiSprite, Context, Key, Game, UpdateResult, rgb};

use crate::{world::World, TEXT_SIZE, player::Player, util::FastRandom};

mod adapter;

pub fn create_world_scene<const SIZE: usize, const H: usize>(world: World<SIZE, H>) -> Game<Player<SIZE, H>> {
    let player = Player { world };
    Game::new(player, get_char_at, on_tick, 2.5)
}

fn get_char_at<const SIZE: usize, const H: usize>(player: &Player<SIZE, H>, x: usize, y: usize) -> AsciiSprite {
    let position = &player.world.position_relative_to_player(
        x as isize - TEXT_SIZE.0 as isize / 2,
        y as isize - TEXT_SIZE.1 as isize / 2,
    );
    let slot = player.world[position];
    match slot.entity {
        Some(e) => e.get_tile_appearance(),
        None => {
            fn random_color_multiplier(random: &mut FastRandom, max_offset: f32) -> f32 {
                1.0 + (random.next_less_than(256) as f32 / 255.0 - 0.5) * max_offset
            }
            let tile = slot.tile.get_tile_appearance();
            let mut random = FastRandom::new(position.uid());
            let max_offset = 0.1;
            let r = random_color_multiplier(&mut random, max_offset);
            let bg = rgb((tile.bg.r * r).min(1.0), (tile.bg.g * r).min(1.0), (tile.bg.b * r).min(1.0));
            let max_offset = 0.2;
            let r = random_color_multiplier(&mut random, max_offset);
            let g = random_color_multiplier(&mut random, max_offset);
            let b = random_color_multiplier(&mut random, max_offset);
            let fg = rgb((tile.fg.r * r).min(1.0), (tile.fg.g * g).min(1.0), (tile.fg.b * b).min(1.0));
            AsciiSprite { bg, fg, ..tile }
        },
    }
}

fn on_tick<const SIZE: usize, const H: usize>(player: &mut Player<SIZE, H>, context: &mut Context, key: Option<Key>) -> UpdateResult {
    match key {
        Some(k) => match k {
            Key::W => { player.step_up(); },
            Key::S => { player.step_down(); },
            Key::A => { player.step_left(); },
            Key::D => { player.step_right(); },
            Key::Space => { player.try_enter(); },
            _ => return UpdateResult::NoChange
        },
        None => {}
    }
    UpdateResult::Update
}