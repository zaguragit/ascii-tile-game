
use engine::{AsciiSprite, Context, Key, Game, UpdateResult, rgb, RGB, util::draw_text, rgb_gray};
use crate::{TEXT_SIZE, player::Player, scene::world::adapter::tile_to_ascii_sprite};
use simulation::{world::{World, Entity}, util::FastRandom};

mod adapter;

pub fn create_world_scene<const SIZE: usize, const H: usize>(world: World<SIZE, H>) -> Game<Player<SIZE, H>> {
    let player = Player::new(world, 24);
    Game::new(player, get_char_at, on_tick, 2.5)
}

fn draw_gui<const SIZE: usize, const H: usize>(
    player: &Player<SIZE, H>,
    x: usize,
    y: usize,
) -> Option<AsciiSprite> {
    const xoff: usize = 1;
    const yoff: usize = 1;
    const bg: RGB = rgb(0.1, 0.1, 0.1);
    const fg: RGB = RGB::WHITE;

    draw_text(player.ambient.to_string().as_str(), x, y, xoff, yoff, bg, fg)
}

fn get_char_at<const SIZE: usize, const H: usize>(
    player: &Player<SIZE, H>,
    x: usize,
    y: usize,
) -> AsciiSprite {
    match draw_gui(player, x, y) {
        Some(c) => return c,
        None => {},
    }

    let position = &player.world.position_relative_to_player(
        x as isize - TEXT_SIZE.0 as isize / 2,
        y as isize - TEXT_SIZE.1 as isize / 2,
    );
    let slot = player.world[position];
    let vision = (get_vision(player, x, y) * 1.4).min(1.0);
    fn random_offset(random: &mut FastRandom, max_offset: f32) -> f32 {
        (random.next_less_than(256) as f32 / 255.0 - 0.5) * max_offset
    }
    let seed = FastRandom::get((position.x) as _) + (FastRandom::get(position.y as _) * 31);
    let mut random = FastRandom::new(seed as _);
    let max_offset = 0.35 - 0.3 * vision;
    let tile = tile_to_ascii_sprite(&slot.tile, &mut random);
    let r = vision + random_offset(&mut random, max_offset);
    let bg = tile.bg * rgb_gray(r);
    match slot.entity {
        Some(e) => {
            let char = match e {
                Entity::Creature(c) => player.world.species[&c.species].symbol,
                Entity::Object(_) => 'o',
            };
            let fg = if bg.squared_perceived_lightness() > 0.25 {
                RGB::BLACK
            } else {
                rgb(vision, vision, vision)
            };
            AsciiSprite { bg, fg, index: char as u8 }
        },
        None => {
            let max_offset = 0.1;
            let r = vision + random_offset(&mut random, max_offset);
            let fg = tile.fg * rgb_gray(r);
            AsciiSprite { bg, fg, ..tile }
        },
    }
}

fn get_vision<const SIZE: usize, const H: usize>(player: &Player<SIZE, H>, x: usize, y: usize) -> f32 {
    let x = x as isize - TEXT_SIZE.0 as isize / 2;
    let y = y as isize - TEXT_SIZE.1 as isize / 2;
    if x.unsigned_abs() > player.radius || y.unsigned_abs() > player.radius {
        0.0
    } else {
        let vsize = player.radius * 2 + 1;
        player.vision[(x + player.radius as isize) as usize * vsize + (y + player.radius as isize) as usize]
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
    player.tick();
    UpdateResult::Update
}