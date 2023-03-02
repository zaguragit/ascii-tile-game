use crate::{world::{World, Entity, creature::Interest, Level, Tile}, util::FastRandom};

pub mod vision;

pub fn tick<const SIZE: usize, const H: usize>(world: &mut World<SIZE, H>, random: &mut FastRandom) {
    let pp = world.player_position;
    for l in 0..H {
        let level = &mut world[l];
        for x in 0..SIZE {
            for y in 0..SIZE {
                let pos = (x, y);
                let slot = &mut level[pos];
                match &mut slot.entity {
                    Some(e) => match e {
                        Entity::Creature(c) => {
                            if pp.x == x && pp.y == y && pp.level == l {
                                continue;
                            }
                            let interest = c.calculate_interests(random);
                            match interest {
                                Interest::Hungry => todo!(),
                                Interest::Scared => todo!(),
                                Interest::Curious => match c.memory.target {
                                    Some(target) => {
                                        let go_to = if target.level != l {
                                            closest_portal_tile(random, level, x, y, 128).unwrap_or((target.x, target.y))
                                        } else {
                                            (target.x, target.y)
                                        };
                                        let direction = (go_to.0 as i64 - x as i64, go_to.1 as i64 - y as i64);
                                        let direction = if direction.0.abs() > direction.1.abs() {
                                            (direction.0.signum(), 0)
                                        } else { (0, direction.1.signum()) };
                                        let new_pos = (
                                            (x as i64 + direction.0 + SIZE as i64) as usize % SIZE,
                                            (y as i64 + direction.1 + SIZE as i64) as usize % SIZE,
                                        );
                                        let new_slot = &level[new_pos];
                                        if new_slot.tile.is_floor() && new_slot.entity.is_none() {
                                            level.swap_entities(pos, new_pos)
                                        }
                                    },
                                    None => {},
                                },
                                Interest::Idle => if random.one_in(16) {
                                    let d = random.next_less_than(4);
                                    let new_pos = match d {
                                        0 => (x, (y + 1) % SIZE),
                                        1 => (x, (y as i64 - 1 + SIZE as i64) as usize % SIZE),
                                        2 => ((x + 1) % SIZE, y),
                                        3 => ((x as i64 - 1 + SIZE as i64) as usize % SIZE, y),
                                        _ => panic!("Random number generator is busted apparently")
                                    };
                                    let new_slot = &level[new_pos];
                                    if new_slot.tile.is_floor() && new_slot.entity.is_none() {
                                        level.swap_entities(pos, new_pos)
                                    }
                                },
                            }
                        },
                        _ => {},
                    },
                    None => {},
                }
            }
        }
    }
    for l in 0..H {
        let level = &mut world[l];
        for x in 0..SIZE {
            for y in 0..SIZE {
                let slot = &mut level[(x, y)];
                match &mut slot.entity {
                    Some(e) => match e {
                        Entity::Creature(c) => {
                            if c.body.health == 0 {
                                println!("hungy??? {:?}", c);
                                todo!("entity death")
                            } else if c.body.nutrition == 0 {
                                c.body.health -= 1;
                            }
                        },
                        _ => {},
                    },
                    None => {},
                }
            }
        }
    }
}

fn closest_portal_tile<const SIZE: usize>(
    random: &mut FastRandom,
    level: &Level<SIZE>,
    x: usize,
    y: usize,
    max_axis_distance: usize,
) -> Option<(usize, usize)> {
    let mut dist = 1;
    'l: loop {
        if random.one_in(2) {
            continue;
        }
        for x in (x as i64 - dist)..=(x as i64 + dist) {
            let c = ((x + SIZE as i64) as usize % SIZE, (y as i64 - dist + SIZE as i64) as usize % SIZE);
            if matches!(level[c].tile, Tile::GotoUnderworld | Tile::GotoOverworld) {
                break 'l Some(c);
            }
            let c = ((x + SIZE as i64) as usize % SIZE, (y as i64 + dist) as usize % SIZE);
            if matches!(level[c].tile, Tile::GotoUnderworld | Tile::GotoOverworld) {
                break 'l Some(c);
            }
        }
        for y in (y as i64 - dist + 1)..(y as i64 + dist) {
            let c = ((x as i64 - dist + SIZE as i64) as usize % SIZE, (y + SIZE as i64) as usize % SIZE);
            if matches!(level[c].tile, Tile::GotoUnderworld | Tile::GotoOverworld) {
                break 'l Some(c);
            }
            let c = ((x as i64 + dist) as usize % SIZE, (y + SIZE as i64) as usize % SIZE);
            if matches!(level[c].tile, Tile::GotoUnderworld | Tile::GotoOverworld) {
                break 'l Some(c);
            }
        }
        if dist as usize >= max_axis_distance {
            break 'l None;
        }
        dist += 1;
    }
}