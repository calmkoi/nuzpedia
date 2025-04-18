use crate::TypeGen1;

// Generation 1 ----------------------------------------------------------
// Definitions -----------------------------------------------------------
#[derive(Debug, Clone)]
pub struct PokemonGen1 {
    pub name: String,
    pub types: [TypeGen1; 2], // e.g., [Type::Electric, Type::None]
    pub stats: StatsGen1,
    pub ivs: IVsGen1,
    pub evs: EVsGen1,
    pub stat_stages: StatStagesGen1,
    pub status: StatusGen1,
}

#[derive(Debug, Clone)]
pub struct IVsGen1 {  // values are 0-15
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special: u8,
    pub speed: u8,
}

// EVs in Gen 1 are 0-65535 range
#[derive(Debug, Clone)]
pub struct EVsGen1 {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special: u16,
    pub speed: u16,
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

// Default Implementations -----------------------------------------------
impl Default for PokemonGen1 {
    fn default() -> Self {
        Self {
            name: String::new(),
            types: [TypeGen1::Normal, TypeGen1::None],
            stats: Default::default(),
            ivs: Default::default(),
            evs: Default::default(),
            stat_stages: Default::default(),
            status: Default::default(),
        }
    }
}

impl Default for IVsGen1 {
    fn default() -> Self {
        Self {
            hp: 0,
            attack: 0,
            defense: 0,
            special: 0,
            speed: 0
        }
    }
}

impl Default for EVsGen1 {
    fn default() -> Self {
        Self {
            hp: 0,
            attack: 0,
            defense: 0,
            special: 0,
            speed: 0
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

// stat calculation helpers for IVs & EVs --------------------------------

impl PokemonGen1 {
    pub fn calculate_stat(&self, base_stat: u8, iv: u8, ev: u16, is_hp: bool) -> u8 {
        // Gen 1 uses √EV in the calculation, not EV/4
        let ev_factor = (ev as f64).sqrt().floor() as u16;
        (((base_stat as u16 + iv as u16) * 2 + ev_factor) * self.stats.lvl as u16 / 100 + if is_hp { 10 } else { 5 }).min(255) as u8
    }

    pub fn recalculate_stats(&mut self) {
        self.stats.hp = self.calculate_stat(
            self.stats.hp, self.ivs.hp, self.evs.hp, true
        );

        self.stats.attack = self.calculate_stat(
            self.stats.attack, self.ivs.attack, self.evs.attack, false
        );

        self.stats.defense = self.calculate_stat(
            self.stats.defense, self.ivs.defense, self.evs.defense, false
        );

        self.stats.special = self.calculate_stat(
            self.stats.special, self.ivs.special, self.evs.special, false
        );

        self.stats.speed = self.calculate_stat(
            self.stats.speed, self.ivs.speed, self.evs.speed, false
        );
    }
}