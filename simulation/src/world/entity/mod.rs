use self::creature::Creature;

pub mod creature;

#[derive(Debug, Copy, Clone)]
pub enum Entity {
    Creature(Creature),
    Object(ObjectType),
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ObjectType {
    Stone,
}