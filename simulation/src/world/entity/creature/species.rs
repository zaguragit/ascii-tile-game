use std::{collections::HashMap};

use crate::util::FastRandom;

use super::{Creature, BehaviorStats, BodyStats, Memory};

pub type SpeciesMap = HashMap<SpeciesID, SpeciesTemplate>;

#[derive(Debug)]
pub struct SpeciesTemplate {
    pub name: String,
    pub symbol: char,
    pub diet: Diet,

    pub max_health: u8,
    pub max_nutrition: u8,
    
    pub smelliness: u8,
    pub strength: u8,
    
    pub awareness: u8,
    pub curiosity: u8,
    pub friendliness: i8,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct SpeciesID(pub u64);

impl PartialEq for SpeciesTemplate {
    fn eq(&self, other: &Self) -> bool { self.name == other.name }
    fn ne(&self, other: &Self) -> bool { self.name != other.name }
}


#[derive(Debug, Copy, Clone)]
pub struct Diet {
    pub meat: bool,
    pub plants: bool,
    pub light: bool,
}

pub fn creature_from_species(
    random: &mut FastRandom,
    species: &SpeciesMap,
    species_id: SpeciesID,
) -> Creature {
    let species_template = species.get(&species_id).unwrap();
    let max_health = species_template.max_health;
    let max_nutrition = species_template.max_nutrition;
    Creature {
        species: species_id,
        behavior: BehaviorStats {
            awareness: species_template.awareness,
            curiosity: species_template.curiosity,
            friendliness: species_template.friendliness,
        },
        body: BodyStats {
            max_health,
            max_nutrition,
            health: max_health,
            nutrition: max_nutrition,
            karma: 0,
            smelliness: species_template.smelliness,
            strength: species_template.strength,
        },
        memory: Memory::new(),
    }
}

pub fn base_creature_from_species(
    species: &SpeciesMap,
    species_id: SpeciesID,
) -> Creature {
    let species_template = species.get(&species_id).unwrap();
    let max_health = species_template.max_health;
    let max_nutrition = species_template.max_nutrition;
    Creature {
        species: species_id,
        behavior: BehaviorStats {
            awareness: species_template.awareness,
            curiosity: species_template.curiosity,
            friendliness: species_template.friendliness,
        },
        body: BodyStats {
            max_health,
            max_nutrition,
            health: max_health,
            nutrition: max_nutrition,
            karma: 0,
            smelliness: species_template.smelliness,
            strength: species_template.strength,
        },
        memory: Memory::new(),
    }
}