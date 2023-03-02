
#[derive(Debug, Copy, Clone)]
pub struct BodyStats {
    /// 0 means death
    pub max_health: u8,
    /// Once reached the maximum, you start to take damage
    pub max_nutrition: u8,

    /// 0 means death
    pub health: u8,
    /// Once 0 is reached, you start to take damage
    pub nutrition: u8,
    /// how much, on average, health is removed from enemies per hit
    pub karma: i8,
    /// can affect how much other creatures avoid you
    pub smelliness: u8,
    /// how much, on average, health is removed from enemies per hit
    pub strength: u8,
}