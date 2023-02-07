use std::{collections::LinkedList, ops::RangeInclusive};

use crate::{world::{Level, Slot, Tile}, util::FastRandom};

#[derive(PartialEq, Eq, Debug)]
struct Room {
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

    fn distance<const WORLD_SIZE: usize>(a: &Self, b: &Self) -> f32 {
        let aw = a.right - a.left;
        let ah = a.bottom - a.top;
        let acx = a.right + a.left;
        let acy = a.bottom + a.top;

        let bw = b.right - b.left;
        let bh = b.bottom - b.top;
        let bcx = b.right + b.left;
        let bcy = b.bottom + b.top;

        let dx = acx.abs_diff(bcx) as i64 - (aw + bw) as i64;
        let dy = acy.abs_diff(bcy) as i64 - (ah + bh) as i64;

        let dx = dx.min((WORLD_SIZE * 2) as i64 - dx);
        let dy = dy.min((WORLD_SIZE * 2) as i64 - dy);

        let dx = dx as f32 / 2.0;
        let dy = dy as f32 / 2.0;

        f32::sqrt(dx * dx + dy * dy)
    }
}

pub(super) fn generate_dungeon<const SIZE: usize>(seed: u64) -> Level<SIZE> {
    let mut level = Level {
        slots: vec![Slot {
            tile: Tile::Void,
            entity: None,
        }; SIZE * SIZE].into_boxed_slice()
    };
    let mut random = FastRandom::new(seed - 4);
    let mut rooms = Vec::<Room>::new();
    let quadrant_size = 64;
    for quadrant_x in 0..(SIZE / quadrant_size) {
        for quadrant_y in 0..(SIZE / quadrant_size) {
            let position_x = quadrant_x * quadrant_size;
            let position_y = quadrant_y * quadrant_size;
            place_rooms(&mut level, &mut random, position_x, position_y, quadrant_size, quadrant_size, 6, &mut rooms);
        }
    }
    'c: for room in &rooms {
        let mut out_connections = random.next_in_range(1, 3);
        let mut max_distance = 1u64;
        let mut already_connected = vec![room];
        'o: loop {
            for other_room in &rooms {
                if !already_connected.contains(&other_room) && Room::distance::<SIZE>(&room, &other_room) < max_distance as f32 && random.one_in(2) {
                    connect_rooms(&mut level, &mut random, room, other_room);
                    already_connected.push(other_room);
                    out_connections -= 1;
                    if out_connections == 0 {
                        break 'o;
                    }
                }
            }
            if max_distance >= 128 {
                break 'c;
            }
            max_distance *= 2;
        }
    }
    level
}

fn connect_rooms<const SIZE: usize>(
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
        level[(x % SIZE, y0 % SIZE)].tile = Tile::DungeonFloor;
        let above = (x % SIZE, (y0 - 1) % SIZE);
        if level[above].tile == Tile::Void {
            level[above].tile = Tile::DungeonWall;
        }
        let below = (x % SIZE, (y0 + 1) % SIZE);
        if level[below].tile == Tile::Void {
            level[below].tile = Tile::DungeonWall;
        }
    }
    for y in yr {
        level[(x1 % SIZE, y % SIZE)].tile = Tile::DungeonFloor;
        let left = ((x1 - 1) % SIZE, y % SIZE);
        if level[left].tile == Tile::Void {
            level[left].tile = Tile::DungeonWall;
        }
        let right = ((x1 + 1) % SIZE, y % SIZE);
        if level[right].tile == Tile::Void {
            level[right].tile = Tile::DungeonWall;
        }
    }
    level[(x1 % SIZE, y0 % SIZE)].tile = Tile::DungeonFloor;
    for x in -1..=1 {
        for y in -1..=1 {
            let corner = ((x1 as i32 + x) as usize % SIZE, (y0 as i32 + y) as usize % SIZE);
            if level[corner].tile == Tile::Void {
                level[corner].tile = Tile::DungeonWall;
            }
        }
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

    if depth == 0 || random.one_in(3 + depth as u64 * 3 / 2) {
        let s = quadrant_width.min(quadrant_height) as u64;
        let min_padding = (s / 5).max(1);
        let max_padding = (s / 3).max(2);
        let (left, top) = (random.next_in_range(min_padding, max_padding + 1), random.next_in_range(min_padding, max_padding + 1));
        let (right, bottom) = (random.next_in_range(min_padding, max_padding + 1), random.next_in_range(min_padding, max_padding + 1));
        
        let left = position_x + left as usize;
        let top = position_y + top as usize;
        let right = position_x + quadrant_width - right as usize;
        let bottom = position_y + quadrant_height - bottom as usize;

        for x in left..=right {
            level[(x as usize % SIZE, top as usize % SIZE)].tile = Tile::DungeonWall;
            level[(x as usize % SIZE, bottom as usize % SIZE)].tile = Tile::DungeonWall;
        }
        for y in (top + 1)..bottom {
            level[(left as usize % SIZE, y as usize % SIZE)].tile = Tile::DungeonWall;
            level[(right as usize % SIZE, y as usize % SIZE)].tile = Tile::DungeonWall;
        }

        for x in (left + 1)..right {
            for y in (top + 1)..bottom {
                let c = (x as usize % SIZE, y as usize % SIZE);
                level[c].tile = Tile::DungeonFloor;
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