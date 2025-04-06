use crate::TypeGen1;

// Generation 1 ----------------------------------------------------------
#[derive(Debug, Clone)]
pub struct PokemonGen1 {
    pub name: String,
    pub types: [TypeGen1; 2], // e.g., [Type::Electric, Type::None]
    pub stats: StatsGen1,
    pub stat_stages: StatStagesGen1,
    pub status: StatusGen1,
}

#[derive(Debug, Clone)]
pub struct StatsGen1 {
    pub lvl: u8,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special: u8,  // Gen 1 uses one special stat
    pub speed: u8,
}

#[derive(Debug, Clone)]
pub struct StatStagesGen1 { // Apply this in damage.rs?
    pub attack: i8, // -6 to +6
    pub defense: i8,
    pub special: i8,
    pub speed: i8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusGen1 {
    Healthy,
    Burned,
    Poisoned,
    Paralyzed,
    Asleep(u8),
    Frozen,
    // Note that Gen 1 doesn't have the badly poisoned condition
}

// TODO: Implement other status effects:
// - Sleep/Frozen: Prevent attacking
// - Paralyze: Speed reduction and attack failure chance
// - Poison: End-of-turn damage
// - Badly Poisoned: Progressive damage (Gen 2+)

impl Default for PokemonGen1 {
    fn default() -> Self {
        Self {
            name: String::new(),
            types: [TypeGen1::Normal, TypeGen1::None],
            stats: Default::default(),
            stat_stages: Default::default(),
            status: Default::default(),
        }
    }
}

impl Default for StatsGen1 {
    fn default() -> Self {
        Self { 
            lvl: 100, 
            hp: 0, 
            attack: 0, 
            defense: 0, 
            special: 0, 
            speed: 0 
        }
    }
}

impl Default for StatStagesGen1 {
    fn default() -> Self {
        Self { 
            attack: 0, 
            defense: 0, 
            special: 0, 
            speed: 0 
        }
    }
}

impl Default for StatusGen1 {
    fn default() -> Self {
        StatusGen1::Healthy
    }
}