use crate::{PokemonGen1, MoveGen1, MoveCategory};

pub fn calc_damage_gen_1(
    attacker: &PokemonGen1,
    defender: &PokemonGen1,
    mov: &MoveGen1,
    is_critical: bool,
) -> u16 {
    // Handle status moves (0 damage)
    if mov.power == 0 {
        return 0;
    }

    // Determine STAB
    let stab: f64 = if attacker.types.contains(&mov.typ) { 1.5 } else { 1.0 };

    // Determine Crit
    let crit: u32 = if is_critical { 2 } else { 1 };

    // Use Attack or Special based on move type
    let (attacker_stat, defender_stat) = match mov.category {
        MoveCategory::Physical => (attacker.stats.attack, defender.stats.defense),
        MoveCategory::Special => (attacker.stats.special, defender.stats.special),
        MoveCategory::Status => unreachable!(), // Handled by power check
    };

    // TODO: implement stat boosts and add logic for crits to ignore them

    // Convert values to u32 to avoid overflow
    let lvl: u32 = attacker.stats.lvl as u32;
    let power: u32 = mov.power as u32;
    let attack: u32 = attacker_stat as u32;
    let defense: u32 = defender_stat as u32;

    // Base damage formula (excl. random)
    let base: u32 = (((2 * lvl * crit / 5 + 2) * power * attack) / (defense * 50) + 2) as u32;

    // Apply STAB and TODO: Add type chart
    let type_eff = 1.0; // Placeholder for type effectiveness
    let damage = (base as f64 * stab * type_eff) as u16;

    // Apply random factor: Gen 1 rolls 217-255 (85-100% of damage)
    // TODO: implement options for high and low rolls
    let random_factor = 217 + (rand::random::<u8>() % 39); // [217, 255]
    (damage * random_factor as u16) / 255
}

#[cfg(test)]
mod tests {
    use super::*;
    // Need to explicitly import these as this file doesn't use them
    use crate::types::TypeGen1;
    use crate::StatsGen1;

    #[test]
    fn test_gen1_damage() {
        let pikachu = PokemonGen1 {
            name: "Pikachu".into(),
            types: [TypeGen1::Electric, TypeGen1::None],
            stats: StatsGen1 {lvl: 100, hp: 35, attack: 55, defense: 30, special: 50, speed: 90},
        };
        let thunderbolt = MoveGen1 {
            name: "Thunderbolt".into(),
            typ: TypeGen1::Electric,
            power: 95,
            category: MoveCategory::Special,
        };
        assert!(calc_damage_gen_1(&pikachu, &pikachu, &thunderbolt, false) > 0); // TODO: fix this to actually test properly
    }
}