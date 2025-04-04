use crate::{Pokemon, Move, MoveCategory, Type, Stats};

pub fn calculate_gen1(
    attacker: &Pokemon,
    defender: &Pokemon,
    mov: &Move,
) -> u16 {
    // Determine STAB
    let stab = if attacker.types.contains(&mov.typ) { 1.5 } else { 1.0 };

    // TODO: logic to handle status moves
    // Use Attack or Special based on move type
    let attacker_attack = if mov.category == MoveCategory::Physical {
        attacker.stats.attack
    } else {
        attacker.stats.special
    };

    // Convert values to u32 before multiplication
    let lvl = attacker.stats.lvl as u32;
    let power = mov.power as u32;
    let attack = attacker_attack as u32;
    let defense = defender.stats.defense as u32;

    // Base damage formula (excl. crit and random)
    let base = (((2 * lvl / 5 + 2) * power * attack) / (defense * 50) + 2) as u32;


    (base as f64 * stab) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen1_damage() {
        let pikachu = Pokemon {
            name: "Pikachu".into(),
            types: [Type::Electric, Type::None],
            stats: Stats {lvl: 100, hp: 35, attack: 55, defense: 30, special: 50, speed: 90},
        };
        let thunderbolt = Move {
            name: "Thunderbolt".into(),
            typ: Type::Electric,
            power: 95,
            category: MoveCategory::Special,
        };
        assert_eq!(calculate_gen1(&pikachu, &pikachu, &thunderbolt), 202); // Example value
    }
}