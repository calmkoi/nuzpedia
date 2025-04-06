use crate::{PokemonGen1, MoveGen1, MoveCategory};
use crate::types::type_effectiveness_gen_1;

/// Damage calculation options for controlling RNG behavior
///
/// In Gen 1, damage rolls range from 85% to 100% of base damage:
/// - Min: 217/255 (85%)
/// - Average: 236/255 (~92.5%)
/// - Max: 255/255 (100%)
/// - Random: Random value in [217,255]
#[derive(Debug, Clone, Copy)]
pub enum DamageRoll {
    Min,
    Average,
    Max,
    Random,
}

pub fn calc_damage_gen_1(
    attacker: &PokemonGen1,
    defender: &PokemonGen1,
    mov: &MoveGen1,
    is_critical: bool,
    roll: DamageRoll,
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
    let type_eff = type_effectiveness_gen_1(
        mov.typ,
        &defender.types
    );
    let damage = (base as f64 * stab * type_eff) as u16;

    // Apply random factor: Gen 1 rolls 217-255 (85-100% of damage)
    let damage: u16 = match roll {
        DamageRoll::Min => (damage * 217) / 255,
        DamageRoll::Average => (damage * 236) / 255,
        DamageRoll::Max => damage,
        DamageRoll::Random => {
            let random_factor = 217 + (rand::random::<u8>() % 39);
            (damage * random_factor as u16) / 255
        }
    };

    return damage;
}

#[cfg(test)]
mod tests {
    use super::*;
    // Need to explicitly import these as this file doesn't use them
    use crate::types::TypeGen1;
    use crate::StatsGen1;

    #[test]
    fn test_thunderbolt_vs_starmie() {
        let pikachu = PokemonGen1 {
            name: "Pikachu".into(),
            types: [TypeGen1::Electric, TypeGen1::None],
            stats: StatsGen1 {
                lvl: 50,
                hp: 35,
                attack: 55,
                defense: 30,
                special: 50,
                speed: 90,
            },
        };
        
        let starmie = PokemonGen1 {
            name: "Starmie".into(),
            types: [TypeGen1::Water, TypeGen1::Psychic],
            stats: StatsGen1 {
                lvl: 50,
                hp: 60,
                attack: 75,
                defense: 85,
                special: 95,
                speed: 115,
            },
        };

        let thunderbolt = MoveGen1 { 
            name: "Thunderbolt".into(), 
            typ: TypeGen1::Electric, 
            power: 90, 
            category: MoveCategory::Special,
        };
        
        // Test consistent rolls
        let min = calc_damage_gen_1(&pikachu, &starmie, &thunderbolt, false, DamageRoll::Min);
        let avg = calc_damage_gen_1(&pikachu, &starmie, &thunderbolt, false, DamageRoll::Average);
        let max = calc_damage_gen_1(&pikachu, &starmie, &thunderbolt, false, DamageRoll::Max);

        assert!(min < avg && avg < max); 
        
        // Test random roll falls within acceptable bounds
        let random = calc_damage_gen_1(&pikachu, &starmie, &thunderbolt, false, DamageRoll::Random);

        assert!(random >= min && random <= max);
        
    }
}