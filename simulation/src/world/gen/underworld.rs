
use crate::{world::{Level, Slot, Tile}, util::FastRandom};

use super::{open_simplex_tileable_3d::OpenSimplexTileable3D, carve_from};

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Room {
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

impl Room {
    fn random_position_inside(&self, random: &mut FastRandom) -> (usize, usize) {
        (
            random.next_in_range(self.left as u64 + 1, self.right as u64) as usize,
            random.next_in_range(self.top as u64 + 1, self.bottom as u64) as usize,
        )
    }

    fn square_distance<const WORLD_SIZE: usize>(a: &Self, b: &Self) -> isize {
        let aw = a.right - a.left;
        let ah = a.bottom - a.top;
        let acx = a.right + a.left;
        let acy = a.bottom + a.top;

        let bw = b.right - b.left;
        let bh = b.bottom - b.top;
        let bcx = b.right + b.left;
        let bcy = b.bottom + b.top;

        let dx = acx.abs_diff(bcx) as isize - (aw + bw) as isize;
        let dy = acy.abs_diff(bcy) as isize - (ah + bh) as isize;

        let dx = dx.min((WORLD_SIZE * 2) as isize - dx);
        let dy = dy.min((WORLD_SIZE * 2) as isize - dy);

        (dx * dx + dy * dy) / 4
    }
}

pub(super) fn generate_underworld<const SIZE: usize>(seed: u64, quadrant_size: usize) -> (Level<SIZE>, Vec<Room>) {
    let mut level = Level {
        slots: vec![Slot {
            tile: Tile::Void,
            entity: None,
        }; SIZE * SIZE].into_boxed_slice()
    };
    let mut random = FastRandom::new(seed - 4);
    place_caves(&mut level, &mut random);
    let rooms = place_dungeons(&mut level, &mut random, quadrant_size);
    carve_worm_caves(&mut level, &mut random);
    (level, rooms)
}

fn place_dungeons<const SIZE: usize>(
    level: &mut Level<SIZE>,
    random: &mut FastRandom,
    quadrant_size: usize,
) -> Vec<Room> {
    let mut rooms = Vec::<Room>::new();
    let quadrants_in_side = SIZE / quadrant_size;
    for quadrant_x in 0..quadrants_in_side {
        for quadrant_y in 0..quadrants_in_side {
            if !random.one_in(6) {
                let position_x = quadrant_x * quadrant_size;
                let position_y = quadrant_y * quadrant_size;
                place_rooms(level, random, position_x, position_y, quadrant_size, quadrant_size, 4, &mut rooms);
            }
        }
    }
    connect_rooms(level, &rooms, random);
    rooms
}

fn place_caves<const SIZE: usize>(
    level: &mut Level<SIZE>,
    random: &mut FastRandom,
) {
    let s = SIZE as i32 / 6;
    let os32 = OpenSimplexTileable3D::new_with_seed_square(random.next() as i64, s / 64);
    let os16 = OpenSimplexTileable3D::new_with_seed_square(random.next() as i64, s / 16);
    let os8 = OpenSimplexTileable3D::new_with_seed_square(random.next() as i64, s / 8);
    for xi in 0..SIZE {
        for yi in 0..SIZE {
            const MIN_WALL_DENSITY: f64 = -0.1;
            const TEMPERATURE_BEACH_WEIGHT: f64 = 0.4;
            let (x, y) = (xi as f64, yi as f64);
            let density_frequency = (os32.eval(x / 32.0, y / 32.0, 192.0) * 0.5 + 0.5).powi(2);
            let inv_density_frequency = 1.0 - density_frequency;
            let weights = [
                1.0 * inv_density_frequency + 0.0 * density_frequency,
                0.0 * inv_density_frequency + 0.6 * density_frequency,
                0.0 * inv_density_frequency + 0.4 * density_frequency,
            ];
            let density = 0.1 + os32.eval(x / 32.0, y / 32.0, 0.0) * weights[0];
            let density = if
                density + weights[1] + weights[2] < MIN_WALL_DENSITY ||
                density - weights[1] - weights[2] > TEMPERATURE_BEACH_WEIGHT {
                    density
                } else { density + os16.eval(x / 16.0, y / 16.0, 0.0) * weights[1] };
            let density = if
                density + weights[2] < MIN_WALL_DENSITY ||
                density - weights[2] > TEMPERATURE_BEACH_WEIGHT {
                    density
                } else { density + os8.eval(x / 8.0, y / 8.0, 0.0) * weights[2] };

            level[(xi, yi)].tile = if density < MIN_WALL_DENSITY {
                let humidity = os32.eval(x / 32.0, y / 32.0, 256.0) * 0.5 + 0.5;
                let humidity = 0.8 * humidity + 0.2 * (os8.eval(x / 8.0, y / 8.0, 256.0) * 0.5 + 0.5);
                if humidity > 0.7 {
                    Tile::Grass
                } else if humidity > 0.5 {
                    Tile::ThinGrass
                } else {
                    Tile::Stone
                }
            } else {
                Tile::Void
            };
        }
    }
}

fn carve_worm_caves<const SIZE: usize>(
    level: &mut Level<SIZE>,
    random: &mut FastRandom,
) {
    let sparceness = 24;
    for _ in 0..(SIZE / sparceness * SIZE / sparceness) {
        let (from_x, from_y) = find_void(random, level);
        let max_distance = 256;
        if let Some((to_x, to_y)) = closest_floor_tile(random, level, from_x, from_y, max_distance) {
            carve_from(level, random, max_distance, from_x, from_y, to_x, to_y, Tile::Stone, true);
        }
    }
    let sparceness = 32;
    for _ in 0..(SIZE / sparceness * SIZE / sparceness) {
        let (from_x, from_y) = level.find_floor(random);
        let (to_x, to_y) = level.find_floor(random);
        let max_distance = 96;
        carve_from(level, random, max_distance, from_x, from_y, to_x, to_y, Tile::Stone, true);
    }
}

fn find_void<const SIZE: usize>(
    random: &mut FastRandom,
    level: &Level<SIZE>,
) -> (usize, usize) {
    loop {
        let (x, y) = (random.next_less_than(SIZE as u64), random.next_less_than(SIZE as u64));
        let c = (x as usize, y as usize);
        let s = level[c];
        if matches!(s.tile, Tile::Void) && s.entity.is_none() {
            return c;
        }
    }
}

fn closest_floor_tile<const SIZE: usize>(
    random: &mut FastRandom,
    level: &Level<SIZE>,
    x: usize,
    y: usize,
    max_axis_distance: usize,
) -> Option<(usize, usize)> {
    let mut dist = 1;
    'l: loop {
        if !random.one_in(32) {
            continue;
        }
        for x in (x as i64 - dist)..=(x as i64 + dist) {
            let c = ((x + SIZE as i64) as usize % SIZE, (y as i64 - dist + SIZE as i64) as usize % SIZE);
            if level[c].tile.is_floor() {
                break 'l Some(c);
            }
            let c = ((x + SIZE as i64) as usize % SIZE, (y as i64 + dist) as usize % SIZE);
            if level[c].tile.is_floor() {
                break 'l Some(c);
            }
        }
        for y in (y as i64 - dist + 1)..(y as i64 + dist) {
            let c = ((x as i64 - dist + SIZE as i64) as usize % SIZE, (y + SIZE as i64) as usize % SIZE);
            if level[c].tile.is_floor() {
                break 'l Some(c);
            }
            let c = ((x as i64 + dist) as usize % SIZE, (y + SIZE as i64) as usize % SIZE);
            if level[c].tile.is_floor() {
                break 'l Some(c);
            }
        }
        if dist as usize >= max_axis_distance {
            break 'l None;
        }
        dist += 1;
    }
}


fn connect_rooms<const SIZE: usize>(
    level: &mut Level<SIZE>,
    rooms: &Vec<Room>,
    random: &mut FastRandom,
) {
    let mut rooms_to_connect: Vec<Room> = rooms.into_iter().map(|r| *r).collect();
    let mut i: isize = 0;
    while i < rooms_to_connect.len() as isize {
        let room = rooms_to_connect[i as usize];
        let out_connections = random.next_in_range(1, 12) as usize;
        let mut closest_distance = 256;
        let mut closest: Vec<&Room> = vec![];
        for other_room in rooms {
            let new_distance = Room::square_distance::<SIZE>(&room, &other_room);
            if new_distance <= closest_distance {
                closest_distance = new_distance;
                closest.push(other_room);
                if closest.len() >= out_connections {
                    closest.remove(0);
                }
            }
        }
        closest.reverse();
        for ci in 0..closest.len() {
            if random.one_in((ci as i64 - 1).max(1) as u64) {
                let other_room = closest[ci];
                let a = rooms_to_connect.iter().position(|r| r == other_room);
                match a {
                    Some(a) => {
                        rooms_to_connect.remove(a);
                        if a as isize <= i {
                            i -= 1;
                        }
                    },
                    _ => {},
                }
                connect_room_pair(level, random, &room, other_room);
            }
        }
        i += 1;
    }
}

fn connect_room_pair<const SIZE: usize>(
    level: &mut Level<SIZE>,
    random: &mut FastRandom,
    a: &Room,
    b: &Room,
) {
    let (x0, y0) = a.random_position_inside(random);
    let (x1, y1) = b.random_position_inside(random);
    let xr = if x0 < x1 { x0..x1 } else { x1..x0 };
    let yr = if y0 < y1 { y0..y1 } else { y1..y0 };
    for x in xr {
        let c = (x % SIZE, y0 % SIZE);
        if matches!(level[c].tile, Tile::Void) {
            level[c].tile = Tile::Stone;
        }
    }
    for y in yr {
        let c = (x1 % SIZE, y % SIZE);
        if matches!(level[c].tile, Tile::Void) {
            level[c].tile = Tile::Stone;
        }
    }
    let c = (x1 % SIZE, y0 % SIZE);
    if matches!(level[c].tile, Tile::Void) {
        level[c].tile = Tile::Stone;
    }
}

fn place_rooms<const SIZE: usize>(
    level: &mut Level<SIZE>,
    random: &mut FastRandom,
    position_x: usize,
    position_y: usize,
    quadrant_width: usize,
    quadrant_height: usize,
    depth: usize,
    accumulator: &mut Vec<Room>
) {
    if random.one_in(4 + depth as u64) {
        return;
    }

    if depth == 0 || random.one_in(10 + depth as u64 * 2) {
        let s = quadrant_width.min(quadrant_height) as u64;
        let min_padding = (s / 5).max(1);
        let max_padding = (s / 3).max(2);
        let (left, top) = (random.next_in_range(min_padding, max_padding + 1), random.next_in_range(min_padding, max_padding + 1));
        let (right, bottom) = (random.next_in_range(min_padding, max_padding + 1), random.next_in_range(min_padding, max_padding + 1));
        
        let left = position_x + left as usize;
        let top = position_y + top as usize;
        let right = position_x + quadrant_width - right as usize;
        let bottom = position_y + quadrant_height - bottom as usize;

        if !(
            matches!(level[(left, top)].tile, Tile::Void) &&
            matches!(level[(right, top)].tile, Tile::Void) &&
            matches!(level[(left, bottom)].tile, Tile::Void) &&
            matches!(level[(right, bottom)].tile, Tile::Void)
        ) { return; }

        for x in left..=right {
            level[(x as usize % SIZE, top as usize % SIZE)].tile = Tile::Void;
            level[(x as usize % SIZE, bottom as usize % SIZE)].tile = Tile::Void;
        }
        for y in (top + 1)..bottom {
            level[(left as usize % SIZE, y as usize % SIZE)].tile = Tile::Void;
            level[(right as usize % SIZE, y as usize % SIZE)].tile = Tile::Void;
        }

        for x in (left + 1)..right {
            for y in (top + 1)..bottom {
                let c = (x as usize % SIZE, y as usize % SIZE);
                level[c].tile = Tile::Stone;
            }
        }

        accumulator.push(Room { left, top, right, bottom });
    } else {
        if quadrant_height > quadrant_width || (quadrant_height == quadrant_width && random.one_in(2)) {
            let quadrant_height = quadrant_height / 2;
            place_rooms(level, random, position_x, position_y, quadrant_width, quadrant_height, depth - 1, accumulator);
            let position_y = position_y + quadrant_height;
            place_rooms(level, random, position_x, position_y, quadrant_width, quadrant_height, depth - 1, accumulator);
        } else {
            let quadrant_width = quadrant_width / 2;
            place_rooms(level, random, position_x, position_y, quadrant_width, quadrant_height, depth - 1, accumulator);
            let position_x = position_x + quadrant_width;
            place_rooms(level, random, position_x, position_y, quadrant_width, quadrant_height, depth - 1, accumulator);
        }
    }
}