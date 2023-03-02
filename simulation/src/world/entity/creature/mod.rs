use crate::util::FastRandom;

mod behavior;
mod body;
mod species;

pub use self::{body::*, behavior::*, species::*};

#[derive(Debug, Copy, Clone)]
pub struct Creature {
    pub species: SpeciesID,
    pub body: BodyStats,
    pub behavior: BehaviorStats,
    pub memory: Memory,
}

impl Creature {
    pub fn calculate_interests(&mut self, random: &mut FastRandom) -> Interest {
        if self.body.health <= (self.body.max_health / 4).max(1) {
            if !matches!(self.memory.current_interest, Interest::Scared) {
                self.memory.last_interest = self.memory.current_interest;
                self.memory.current_interest = Interest::Scared;
            }
        } else if self.body.nutrition as usize <= (self.body.max_nutrition as usize * 2 / 3).max(1) {
            if !matches!(self.memory.current_interest, Interest::Hungry) {
                self.memory.last_interest = self.memory.current_interest;
                self.memory.current_interest = Interest::Hungry;
            }
        } else {
            if matches!(self.memory.current_interest, Interest::Idle) && self.behavior.curiosity > random.next_less_than(256) as u8 {
                self.memory.last_interest = self.memory.current_interest;
                self.memory.current_interest = Interest::Curious;
            } else {
                self.memory.current_interest = self.memory.last_interest;
            }
        }
        self.memory.current_interest
    }
}