use crate::TypeGen1;

// Generation 1 ----------------------------------------------------------
#[derive(Debug, Clone)]
pub struct PokemonGen1 {
    pub name: String,
    pub types: [TypeGen1; 2], // e.g., [Type::Electric, Type::None]
    pub stats: StatsGen1,
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