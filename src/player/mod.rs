use simulation::{world::{World, SlotPosition, Tile, Ambient}, life::{vision::look, tick}, util::FastRandom};

pub struct Player<const SIZE: usize, const H: usize> {
    pub world: World<SIZE, H>,
    pub ambient: Ambient,
    pub vision: Vec<f32>,
    pub radius: usize,
}

impl<const SIZE: usize, const H: usize> Player<SIZE, H> {

    pub fn new(world: World<SIZE, H>, radius: usize) -> Self {
        let pos = world.player_position;
        let ambient = world.get_ambient_at(&pos, radius);
        let vision = look(&world[pos.level], pos.x, pos.y, radius);
        Self {
            world,
            ambient,
            vision,
            radius,
        }
    }

    fn move_to(&mut self, position: SlotPosition) {
        let old_pos = self.world.player_position;
        self.world[&old_pos].entity.expect("Player out of sync with tracked position");
        self.world.swap_entities(old_pos, position);
        self.world.player_position = position;
    }

    fn try_tp<'a>(&'a mut self, xoff: isize, yoff: isize) -> bool {
        let new_pos = self.world.position_relative_to_player(xoff, yoff);

        let next_slot = &self.world[&new_pos];
        if (next_slot.tile.is_floor() || next_slot.tile.is_swimmable()) && matches!(next_slot.entity, None) {
            self.move_to(new_pos);
            true
        } else { false }
    }

    pub fn try_enter(&mut self) -> bool {
        let old_pos = &self.world.player_position;
        match self.world[old_pos].tile {
            Tile::GotoOverworld => {
                self.move_to(old_pos.to_level(0));
                true
            },
            Tile::GotoUnderworld => {
                self.move_to(old_pos.to_level(1));
                true
            },
            _ => false,
        }
    }

    pub fn step_up(&mut self) -> bool { self.try_tp(0, -1) }
    pub fn step_down(&mut self) -> bool { self.try_tp(0, 1) }
    pub fn step_left(&mut self) -> bool { self.try_tp(-1, 0) }
    pub fn step_right(&mut self) -> bool { self.try_tp(1, 0) }

    pub fn tick(&mut self) {
        let pos = self.world.player_position;
        self.ambient = self.world.get_ambient_at(&pos, self.radius);
        self.vision = look(&self.world[pos.level], pos.x, pos.y, self.radius);
        let mut random = FastRandom::new_from_sys_time();
        tick(&mut self.world, &mut random);
    }
}