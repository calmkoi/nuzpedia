#[derive(Debug, Clone)]
pub struct Pokemon {
    pub name: String,
    pub types: [Type; 2], // e.g., [Type::Electric, Type::None]
    pub stats: Stats,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub lvl: u8,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special: u8,  // Gen 1 uses one special stat
    pub speed: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    None,    // For single-type pokemon
}