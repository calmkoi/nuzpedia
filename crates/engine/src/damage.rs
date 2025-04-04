use crate::{Pokemon, Move, MoveCategory, Type, Stats};

pub fn calculate_gen1(
    attacker: &Pokemon,
    defender: &Pokemon,
    mov: &Move,
) -> u16 {
    // Determine STAB
    let stab = if attacker.types.contains(&mov.typ) {1.5} else {1.0};

    // TODO: logic to handle status moves
    // Use Attack or Special based on move type
    let attacker_attack = if mov.category == MoveCategory::Physical {attacker.stats.attack} else {attacker.stats.special};

    // Base damage formula (excl. crit and random)
    let base = ((2 * attacker.stats.lvl / 5 + 2) * mov.power * attacker_attack / defender.stats.defense) / 50 + 2;

    (base as f64 * stab) as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gn1_damage() {
        let pikachu = Pokemon {
            name: "Pikachu".into(),
            types: [Type::Electric, Type::None],
            stats: Stats {lvl: 50, hp: 35, attack: 55, defense: 30, special: 50, speed: 90},
        };
        let thunderbolt = Move {
            name: "Thunderbolt".into(),
            typ: Type::Electric,
            power: 95,
            category: MoveCategory::Special,
        };
        assert_eq!(calculate_gen1(&pikachu, &pikachu, &thunderbolt), 42); // Example value
    }
}