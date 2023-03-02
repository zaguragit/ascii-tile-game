use std::fmt::Display;

use crate::world::SlotPosition;


#[derive(Debug, Copy, Clone)]
pub struct BehaviorStats {
    /// How often the entity scans its environment
    pub awareness: u8,
    /// How likely it is to be curious vs idle
    pub curiosity: u8,
    pub friendliness: i8,
    
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Interest {
    Hungry,
    Scared,
    Curious,
    Idle,
}

impl Display for Interest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Interest::Hungry => "hungry",
            Interest::Scared => "scared",
            Interest::Curious => "curious",
            Interest::Idle => "idle",
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Memory {
    /// Where I want to go
    pub target: Option<SlotPosition>,
    /// What I want now
    pub current_interest: Interest,
    /// What I wanna get back to later
    pub last_interest: Interest,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            target: None,
            current_interest: Interest::Idle,
            last_interest: Interest::Idle,
        }
    }
}