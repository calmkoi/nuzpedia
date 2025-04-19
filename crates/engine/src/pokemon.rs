use crate::TypeGen1;

/// Represents a Pokémon in Generation 1, including its stats, types, and battle state.
///
/// # Fields
/// - `name`: The Pokémon's name (e.g., "Pikachu")
/// - `types`: Primary and secondary types (use `TypeGen1::None` if single-typed)
/// - `stats`: Current battle stats (HP, Attack, etc.)
/// - `ivs`: Individual Values (0-15 for each stat)
/// - `evs`: Effort Values (0-65535 for each stat)
/// - `stat_stages`: Current stat modifiers (-6 to +6)
/// - `status`: Current status condition (Burned, Paralyzed, etc.)
///
/// # Examples
/// ```
/// let mut pikachu = PokemonGen1 {
///     name: String::from("Pikachu"),
///     types: [TypeGen1::Electric, TypeGen1::None],
///     stats: StatsGen1 { lvl: 50, ..Default::default() },
///     ..Default::default()
/// };
/// pikachu.recalculate_stats();
/// ```
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

/// Individual Values (IVs) for a Generation 1 Pokémon.
///
/// In Gen 1:
/// - Range: 0-15 for each stat
/// - Determine stat variation at level up
/// - Hidden values set when Pokémon is obtained
#[derive(Debug, Clone)]
pub struct IVsGen1 {  // values are 0-15
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special: u8,
    pub speed: u8,
}

/// Effort Values (EVs) for a Generation 1 Pokémon.
///
/// In Gen 1:
/// - Range: 0-65535 for each stat
/// - Use square root in stat calculation (unlike later gens)
/// - Gained by defeating Pokémon
#[derive(Debug, Clone)]
pub struct EVsGen1 {
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special: u16,
    pub speed: u16,
}

/// Current battle stats for a Generation 1 Pokémon.
///
/// # Notes
/// - `special` handles both Special Attack and Defense (Gen 1 mechanic)
/// - `hp` is calculated separately from other stats
#[derive(Debug, Clone)]
pub struct StatsGen1 {
    pub lvl: u8,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special: u8,  // Gen 1 uses one special stat
    pub speed: u8,
}

/// Active stat stage modifiers during battle.
///
/// In Gen 1:
/// - Range: -6 to +6 for each stat
/// - Affects damage calculation and speed order
/// - Modified by moves like Growl (+1 Defense) or Swords Dance (+2 Attack)
#[derive(Debug, Clone)]
pub struct StatStagesGen1 { // Apply this in damage.rs?
    pub attack: i8, // -6 to +6
    pub defense: i8,
    pub special: i8,
    pub speed: i8,
}

/// Status conditions in Generation 1.
///
/// # Variants
/// - `Healthy`: No status
/// - `Burned`: Halves Attack, damage each turn
/// - `Poisoned`: Damage each turn
/// - `Paralyzed`: 1/4 chance to not attack, speed halved
/// - `Asleep(u8)`: Turns remaining (1-7)
/// - `Frozen`: Can't attack until thawed
///
/// # Gen 1 Quirks
/// - No "badly poisoned" (toxic) in Gen 1
/// - Frozen can only be thawed by Fire moves or Haze
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

// ================= Default Implementations =================

impl Default for PokemonGen1 {
    /// Creates a default Pokémon with:
    /// - Empty name
    /// - Normal type
    /// - Level 100
    /// - All stats/IVs/EVs at 0
    /// - No status conditions
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
    /// Creates IVs with all values at 0 (minimum possible).
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
    /// Creates EVs with all values at 0 (no EV training).
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
    /// Creates stats with:
    /// - Level 100
    /// - All other stats at 0 (will be recalculated)
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
    /// Creates stat stages at neutral (0).
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
    /// Defaults to `Healthy` (no status condition).
    fn default() -> Self {
        StatusGen1::Healthy
    }
}

// ================= Stat Calculation =================

impl PokemonGen1 {
    /// Calculates an individual stat using Gen 1 formulas.
    ///
    /// # Formula
    /// ```
    /// stat = ((base + IV) * 2 + (EV.sqrt())) * level / 100 + 5
    /// HP = ((base + IV) * 2 + (EV.sqrt())) * level / 100 + 10
    /// ```
    ///
    /// # Arguments
    /// - `base_stat`: The species' base stat (e.g., Pikachu's base Speed is 90)
    /// - `iv`: Individual Value (0-15)
    /// - `ev`: Effort Value (0-65535)
    /// - `is_hp`: Whether to calculate HP (uses +10 instead of +5)
    ///
    /// # Returns
    /// The calculated stat value (capped at 255).
    ///
    /// # Example
    /// ```
    /// let pikachu = PokemonGen1::default();
    /// let speed = pikachu.calculate_stat(90, 15, 65535, false);
    /// assert_eq!(speed, 207); // Pikachu's max Speed at level 100
    /// ```
    pub fn calculate_stat(&self, base_stat: u8, iv: u8, ev: u16, is_hp: bool) -> u8 {
        // Gen 1 uses √EV in the calculation, not EV/4
        let ev_factor = (ev as f64).sqrt().floor() as u16;
        (((base_stat as u16 + iv as u16) * 2 + ev_factor) * self.stats.lvl as u16 / 100 + if is_hp { 10 } else { 5 }).min(255) as u8
    }

    /// Recalculates all stats based on current IVs/EVs/level.
    ///
    /// Updates `self.stats` with new values.
    ///
    /// # Example
    /// ```
    /// let mut pikachu = PokemonGen1::default();
    /// pikachu.stats.lvl = 50;
    /// pikachu.recalculate_stats();
    /// ```
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