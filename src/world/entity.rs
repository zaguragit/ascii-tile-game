
#[derive(Debug, Copy, Clone)]
pub enum Entity {
    Creature(CreatureStats),
    Me(CreatureStats),
    Object(ObjectType),
}

#[derive(Debug, Copy, Clone)]
pub struct CreatureStats {
    /// 0 means death
    pub health: u8,
    /// how much, on average, health is removed from enemies per hit
    pub strength: u8,
    /// how much other creatures should avoid you
    pub intimidation: u8,
    /// used in for luck and AI
    pub karma: i8,
}

#[derive(Debug, Copy, Clone)]
pub struct BehaviorStats {
    
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum ObjectType {
    Stone,
}