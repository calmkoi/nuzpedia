use crate::{PokemonGen1, MoveGen1, MoveCategory, StatusGen1};
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
    // Gen 1 uses integer division with truncation toward zero
    let (numerator, denominator) = match stage.clamp(-6, 6) {
        -6 => (2, 8),
        -5 => (2, 7),
        -4 => (2, 6),
        -3 => (2, 5),
        -2 => (2, 4),
        -1 => (2, 3),
        0  => (2, 2),
        1  => (3, 2),
        2  => (4, 2),
        3  => (5, 2),
        4  => (6, 2),
        5  => (7, 2),
        6  => (8, 2),
        _  => unreachable!(),
    };
    
    // Gen 1 calculation: (base * numerator) / denominator
    let result = (base_stat as u32 * numerator) / denominator;
    result.max(1) // Minimum of 1
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
                MoveCategory::Physical => {
                    let mut stat = attacker.stat_stages.attack;
                    if attacker.status == StatusGen1::Burned {
                        stat = stat / 2;
                        stat.max(1)
                    } else {
                        stat
                    }
                },
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

    // Halve defense stat if the move is Selfdestruct or Explosion
    if mov.name == "Selfdestruct" || mov.name == "Explosion" {
        defender_stat = (defender_stat / 2).max(1);
    }

    // Convert values to u32 to avoid overflow
    let lvl: u32 = attacker.stats.lvl as u32;
    let power: u32 = mov.power as u32;
    let attack: u32 = attacker_stat as u32;
    let defense: u32 = defender_stat as u32;

    // Base damage formula (excl. random)
    let base: u32 = (((2 * lvl * crit / 5 + 2) * power * attack) / (defense * 50) + 2) as u32;

    // Apply STAB and type effectiveness
    let type_eff = type_effectiveness_gen_1(
        mov.typ,
        &defender.types
    );
    let damage = (base as f64 * stab * type_eff) as u16;

    // Apply random factor: Gen 1 rolls 217-255 (85-100% of damage)
    let damage: u16 = match roll {
        DamageRoll::Min => ((damage as u32 * 217) / 255) as u16,
        DamageRoll::Average => ((damage as u32 * 236) / 255) as u16,
        DamageRoll::Max => damage,
        DamageRoll::Random => {
            let random_factor = 217 + (rand::random::<u8>() % 39);
            ((damage as u32 * random_factor as u32) / 255) as u16
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
            ..Default::default()
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
            ..Default::default()
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
    fn test_gen_1_stat_modifiers() {
        // Negative stages
        assert_eq!(apply_stat_modifier(100, -1), 66); // 100*2/3 = 66
        assert_eq!(apply_stat_modifier(101, -1), 67); // 101*2/3 = 67
        assert_eq!(apply_stat_modifier(1, -6), 1);    // Clamped to min 1
        
        // Positive stages
        assert_eq!(apply_stat_modifier(100, 1), 150); // 100*3/2
        assert_eq!(apply_stat_modifier(100, 6), 400); // 100*8/2
        
        // Edge cases
        assert_eq!(apply_stat_modifier(255, 6), 1020); // Max possible
        assert_eq!(apply_stat_modifier(0, 6), 1);      // Clamped from 0
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

    #[test]
    fn test_burn_penalty() {
        let charizard = PokemonGen1 {
            stats: StatsGen1 { attack: 100, ..Default::default() },
            status: StatusGen1::Burned,
            ..Default::default()
        };
        
        let tackle = MoveGen1 {
            category: MoveCategory::Physical,
            power: 40,
            ..Default::default()
        };
        
        // Test consistent rolls
        let min = calc_damage_gen_1(&charizard, &PokemonGen1::default(), &tackle, false, DamageRoll::Min);
        let avg = calc_damage_gen_1(&charizard, &PokemonGen1::default(), &tackle, false, DamageRoll::Average);
        let max = calc_damage_gen_1(&charizard, &PokemonGen1::default(), &tackle, false, DamageRoll::Max);

        assert!(min < avg && avg < max);
    }
}