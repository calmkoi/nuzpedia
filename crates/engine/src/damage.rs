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

fn apply_stat_modifier(base_stat: u8, stage: i8) -> u32 {
    // Gen 1 uses different modifiers than later generations
    let modifier = match stage.clamp(-6, 6) {
        -6 => 2.0 / 8.0,
        -5 => 2.0 / 7.0,
        -4 => 2.0 / 6.0,
        -3 => 2.0 / 5.0,
        -2 => 2.0 / 4.0,
        -1 => 2.0 / 3.0,
        0 => 2.0 / 2.0,
        1 => 3.0 / 2.0,
        2 => 4.0 / 2.0,
        3 => 5.0 / 2.0,
        4 => 6.0 / 2.0,
        5 => 7.0 / 2.0,
        6 => 8.0 / 2.0,
        _ => unreachable!(), // Clamped to -6..=6
    };

    // Apply stat modifier and ensure minimum of 1
    ((base_stat as f64 * modifier).round() as u32).max(1)
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
    let (mut attacker_stat, mut defender_stat) = match mov.category {
        MoveCategory::Physical => (attacker.stats.attack, defender.stats.defense),
        MoveCategory::Special => (attacker.stats.special, defender.stats.special),
        MoveCategory::Status => unreachable!(), // Handled by power check
    };

    // Apply stat changes (crits ignore attack drops/defense boosts)
    if !is_critical {
        attacker_stat = apply_stat_modifier(
            attacker_stat, 
            match mov.category {
                MoveCategory::Physical => attacker.stat_stages.attack,
                MoveCategory::Special => attacker.stat_stages.special,
                _ => 0,
            }
        ) as u8;

        defender_stat = apply_stat_modifier(
            defender_stat, 
            match mov.category {
                MoveCategory::Physical => defender.stat_stages.defense,
                MoveCategory::Special => defender.stat_stages.special,
                _ => 0,
            }
        ) as u8;
    }

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
    use crate::pokemon::StatStagesGen1;
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
            stat_stages: Default::default(),
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
            stat_stages: Default::default(),
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

    #[test]
    fn test_stat_modifiers() {
        // Test neutral stage
        assert_eq!(apply_stat_modifier(100, 0), 100);

        // Test positive stages
        assert_eq!(apply_stat_modifier(100, 1), 150); // 1.5x
        assert_eq!(apply_stat_modifier(100, 2), 200); // 2.0x
        assert_eq!(apply_stat_modifier(100, 6), 400); // 4.0x
        
        // Test negative stages
        assert_eq!(apply_stat_modifier(100, -1), 67); // 2/3
        assert_eq!(apply_stat_modifier(100, -2), 50); // 1/2
        assert_eq!(apply_stat_modifier(100, -6), 25); // 1/4
        
        // Test minimum of 1
        assert_eq!(apply_stat_modifier(1, -6), 1);
    }

    #[test]
    fn test_critical_hit_ignores_stages() {
        let attacker = PokemonGen1 {
            stats: StatsGen1 { attack: 100, ..Default::default() },
            stat_stages: StatStagesGen1 { attack: -6, ..Default::default() },
            ..Default::default()
        };

        let defender = PokemonGen1 {
            stats: StatsGen1 { defense: 100, ..Default::default() },
            stat_stages: StatStagesGen1 { defense: 6, ..Default::default() },
            ..Default::default()
        };

        let move_ = MoveGen1 {
            category: MoveCategory::Physical,
            ..Default::default()
        };

        // Critical hit should ignore the -6 attack and +6 defense
        let crit_damage = calc_damage_gen_1(&attacker, &defender, &move_, true, DamageRoll::Max);
        let normal_damage = calc_damage_gen_1(&attacker, &defender, &move_, false, DamageRoll::Max);

        assert!(crit_damage > normal_damage)
    }
}