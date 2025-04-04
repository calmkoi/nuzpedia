use super::{Pokemon, Move};

pub fn calculate_gen1(
    attacker: &Pokemon,
    defender: &Pokemon,
    mov: &Move,
) -> u16 {
    // Determine STAB
    let stab = if attacker.types.contains(&mov.typ) {1.5} else {1.0};

    // TODO: logic to handle status moves
    // Use Attack or Special based on move type
    let attacker_attack = if mov.category == Physical {attacker.stats.attack} else {attacker.stats.special};

    // Base damage formula (excl. crit and random)
    let base = ((2 * attacker.stats.lvl / 5 + 2) * mov.power * attacker_attack / defender.stats.defense) / 50 + 2;

    (base as f64 * stab) as u16
}