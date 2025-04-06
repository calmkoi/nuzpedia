// Generation 1 ----------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TypeGen1 {
    // Note: None is for single-type pokemon
    Normal, Fire, Water, Electric, 
    Grass, Ice, Fighting, Poison, 
    Ground, Flying, Psychic, Bug, 
    Rock, Ghost, Dragon, None,
}
impl TypeGen1 {
    // Returns an iterator over all actual types (excluding None)
    pub fn iter() -> impl Iterator<Item = TypeGen1> {
        [
            TypeGen1::Normal, TypeGen1::Fire, TypeGen1::Water,
            TypeGen1::Electric, TypeGen1::Grass, TypeGen1::Ice, 
            TypeGen1::Fighting, TypeGen1::Poison, TypeGen1::Ground,
            TypeGen1::Flying, TypeGen1::Psychic, TypeGen1::Bug,
            TypeGen1::Rock, TypeGen1::Ghost, TypeGen1::Dragon,
        ].iter().copied()
    }
}

// Precomputed effectiveness tables
const SUPER_EFFECTIVE: [(TypeGen1, TypeGen1); 38] = [
    (TypeGen1::Fire, TypeGen1::Grass),
    (TypeGen1::Fire, TypeGen1::Ice),
    (TypeGen1::Fire, TypeGen1::Bug),
    (TypeGen1::Water, TypeGen1::Fire),
    (TypeGen1::Water, TypeGen1::Ground),
    (TypeGen1::Water, TypeGen1::Rock),
    (TypeGen1::Electric, TypeGen1::Water),
    (TypeGen1::Electric, TypeGen1::Flying),
    (TypeGen1::Grass, TypeGen1::Water),
    (TypeGen1::Grass, TypeGen1::Ground),
    (TypeGen1::Grass, TypeGen1::Rock),
    (TypeGen1::Ice, TypeGen1::Grass),
    (TypeGen1::Ice, TypeGen1::Ground),
    (TypeGen1::Ice, TypeGen1::Flying),
    (TypeGen1::Ice, TypeGen1::Dragon),
    (TypeGen1::Fighting, TypeGen1::Normal),
    (TypeGen1::Fighting, TypeGen1::Ice),
    (TypeGen1::Fighting, TypeGen1::Rock),
    (TypeGen1::Poison, TypeGen1::Grass),
    (TypeGen1::Poison, TypeGen1::Bug),
    (TypeGen1::Ground, TypeGen1::Fire),
    (TypeGen1::Ground, TypeGen1::Electric),
    (TypeGen1::Ground, TypeGen1::Poison),
    (TypeGen1::Ground, TypeGen1::Rock),
    (TypeGen1::Flying, TypeGen1::Grass),
    (TypeGen1::Flying, TypeGen1::Fighting),
    (TypeGen1::Flying, TypeGen1::Bug),
    (TypeGen1::Psychic, TypeGen1::Fighting),
    (TypeGen1::Psychic, TypeGen1::Poison),
    (TypeGen1::Bug, TypeGen1::Grass),
    (TypeGen1::Bug, TypeGen1::Poison),
    (TypeGen1::Bug, TypeGen1::Psychic),
    (TypeGen1::Rock, TypeGen1::Fire),
    (TypeGen1::Rock, TypeGen1::Ice),
    (TypeGen1::Rock, TypeGen1::Flying),
    (TypeGen1::Rock, TypeGen1::Bug),
    (TypeGen1::Ghost, TypeGen1::Ghost),
    (TypeGen1::Dragon, TypeGen1::Dragon),
];

const NOT_VERY_EFFECTIVE: [(TypeGen1, TypeGen1); 38] = [
    (TypeGen1::Normal, TypeGen1::Rock),
    (TypeGen1::Fire, TypeGen1::Fire),
    (TypeGen1::Fire, TypeGen1::Water),
    (TypeGen1::Fire, TypeGen1::Rock),
    (TypeGen1::Fire, TypeGen1::Dragon),
    (TypeGen1::Water, TypeGen1::Water),
    (TypeGen1::Water, TypeGen1::Grass),
    (TypeGen1::Water, TypeGen1::Dragon),
    (TypeGen1::Electric, TypeGen1::Electric),
    (TypeGen1::Electric, TypeGen1::Grass),
    (TypeGen1::Electric, TypeGen1::Dragon),
    (TypeGen1::Grass, TypeGen1::Fire),
    (TypeGen1::Grass, TypeGen1::Grass),
    (TypeGen1::Grass, TypeGen1::Poison),
    (TypeGen1::Grass, TypeGen1::Flying),
    (TypeGen1::Grass, TypeGen1::Bug),
    (TypeGen1::Grass, TypeGen1::Dragon),
    (TypeGen1::Ice, TypeGen1::Water),
    (TypeGen1::Ice, TypeGen1::Ice),
    (TypeGen1::Fighting, TypeGen1::Poison),
    (TypeGen1::Fighting, TypeGen1::Flying),
    (TypeGen1::Fighting, TypeGen1::Psychic),
    (TypeGen1::Fighting, TypeGen1::Bug),
    (TypeGen1::Poison, TypeGen1::Poison),
    (TypeGen1::Poison, TypeGen1::Ground),
    (TypeGen1::Poison, TypeGen1::Rock),
    (TypeGen1::Poison, TypeGen1::Ghost),
    (TypeGen1::Ground, TypeGen1::Grass),
    (TypeGen1::Ground, TypeGen1::Bug),
    (TypeGen1::Flying, TypeGen1::Electric),
    (TypeGen1::Flying, TypeGen1::Rock),
    (TypeGen1::Psychic, TypeGen1::Psychic),
    (TypeGen1::Bug, TypeGen1::Fire),
    (TypeGen1::Bug, TypeGen1::Fighting),
    (TypeGen1::Bug, TypeGen1::Flying),
    (TypeGen1::Bug, TypeGen1::Ghost),
    (TypeGen1::Rock, TypeGen1::Fighting),
    (TypeGen1::Rock, TypeGen1::Ground),
];

const IMMUNE: [(TypeGen1, TypeGen1); 6] = [
    (TypeGen1::Normal, TypeGen1::Ghost),
    (TypeGen1::Electric, TypeGen1::Ground),
    (TypeGen1::Fighting, TypeGen1::Ghost),
    (TypeGen1::Ground, TypeGen1::Flying),
    (TypeGen1::Ghost, TypeGen1::Normal),
    (TypeGen1::Ghost, TypeGen1::Psychic),
];

pub fn type_effectiveness_gen_1(move_type: TypeGen1, defender_types: &[TypeGen1; 2]) -> f64 {
    // Check immunities (attacker perspective)
    for &defender_type in defender_types {
        if IMMUNE.contains(&(move_type, defender_type)) {
            return 0.0;
        }
    }

    let mut multiplier = 1.0;
    for &defender_type in defender_types {
        if defender_type == TypeGen1::None {
            continue;
        }
        if SUPER_EFFECTIVE.contains(&(move_type, defender_type)) {
            multiplier *= 2.0;
        } else if NOT_VERY_EFFECTIVE.contains(&(move_type, defender_type)) {
            multiplier *= 0.5;
        }
    }
    multiplier
}

// Bitmask optimisation

// Helper Macro
macro_rules! mask {
    ($($bit:expr),*) => {
        { 0 $(| (1 << $bit))* }
    };
}

const SUPER_EFFECTIVE_MASK: [u16; 15] = [
    mask!(),            // Normal (0) -> None
    mask!(4, 5, 11),    // Fire (1) -> Grass(4), Ice(5), Bug(11)
    mask!(1, 8, 12),    // Water (2) -> Fire(1), Ground(8), Rock(12)
    mask!(2, 9),        // Electric (3) -> Water(2), Flying(9)
    mask!(2, 8, 12),    // Grass (4) -> Water(2), Ground(8), Rock(12)
    mask!(4, 8, 9, 14), // Ice (5) -> Grass(4), Ground(8), Flying(9), Dragon(14)
    mask!(0, 5, 12),    // Fighting (6) -> Normal(0), Ice(5), Rock(12)
    mask!(4, 11),       // Poison (7) -> Grass(4), Bug(11)
    mask!(1, 3, 7, 12), // Ground (8) -> Fire(1), Electric(3), Poison(7), Rock(12)
    mask!(4, 6, 11),    // Flying (9) -> Grass(4), Fighting(6), Bug(11)
    mask!(6, 7),        // Psychic (10) -> Fighting(6), Poison(7)
    mask!(4, 7, 10),    // Bug (11) -> Grass(4), Poison(7), Psychic(10)
    mask!(1, 5, 9, 11), // Rock (12) -> Fire(1), Ice(5), Flying(9), Bug(11)
    mask!(13),          // Ghost (13) -> Ghost(13)
    mask!(14),          // Dragon (14) -> Dragon(14)
];

const NOT_VERY_EFFECTIVE_MASK: [u16; 15] = [
    mask!(12),                  // Normal (0) -> Rock(12)
    mask!(1, 2, 12, 14),        // Fire (1) -> Fire(1), Water(2), Rock(12), Dragon(14)
    mask!(2, 4, 14),            // Water (2) -> Water(2), Grass(4), Dragon(14)
    mask!(3, 4, 14),            // Electric (3) -> Electric(3), Grass(4), Dragon(14)
    mask!(1, 4, 7, 9, 11, 14),  // Grass (4) -> Fire(1), Grass(4), Poison(7), Flying(9), Bug(11), Dragon(14)
    mask!(2, 5),                // Ice (5) -> Water(2), Ice(5)
    mask!(7, 9, 10, 11),        // Fighting (6) -> Poison(7), Flying(9), Psychic(10), Bug(11)
    mask!(7, 8, 12, 13),        // Poison (7) -> Poison(7), Ground(8), Rock(12), Ghost(13)
    mask!(4, 11),               // Ground (8) -> Grass(4), Bug(11)
    mask!(3, 12),               // Flying (9) -> Electric(3), Rock(12)
    mask!(10),                  // Psychic (10) -> Psychic(10)
    mask!(1, 6, 9, 13),         // Bug (11) -> Fire(1), Fighting(6), Flying(9), Ghost(13)
    mask!(6, 8),                // Rock (12) -> Fighting(6), Ground(8)
    mask!(13),                  // Ghost (13) -> Ghost(13)
    mask!(14),                  // Dragon (14) -> Dragon(14)
];

const IMMUNE_MASK: [u16; 15] = [
    mask!(13),          // Normal (0) -> Ghost(13)
    mask!(),            // Fire (1) -> None
    mask!(),            // Water (2) -> None
    mask!(8),           // Electric (3) -> Ground(8)
    mask!(),            // Grass (4) -> None
    mask!(),            // Ice (5) -> None
    mask!(13),          // Fighting (6) -> Ghost(13)
    mask!(),            // Poison (7) -> None
    mask!(9),           // Ground (8) -> Flying(9)
    mask!(),            // Flying (9) -> None
    mask!(),            // Psychic (10) -> None
    mask!(),            // Bug (11) -> None
    mask!(),            // Rock (12) -> None
    mask!(0, 10),       // Ghost (13) -> Normal(0), Psychic(10)
    mask!(),            // Dragon (14) -> None
];

pub fn type_effectiveness_gen_1_fast(move_type: TypeGen1, defender_types: &[TypeGen1; 2]) -> f64 {
    let move_idx = move_type as usize;

    // Check immunity (attacker perspective)
    for &defender_type in defender_types {
        if defender_type != TypeGen1::None {
            let def_idx = defender_type as usize;
            if (IMMUNE_MASK[move_idx] & (1 << def_idx)) != 0 {
                return 0.0;
            }
        }
    }

    let mut multiplier = 1.0;
    for &defender_type in defender_types {
        if defender_type == TypeGen1::None {
            continue;
        }
        let def_idx = defender_type as usize;

        if (SUPER_EFFECTIVE_MASK[move_idx] & (1 << def_idx)) != 0 {
            multiplier *= 2.0;
        } else if (NOT_VERY_EFFECTIVE_MASK[move_idx] & (1 << def_idx)) != 0 {
            multiplier *= 0.5;
        }
    }
    multiplier
}

// Generations 2-5 -------------------------------------------------------

// Generation 6-9 --------------------------------------------------------

// Testing ---------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to test two different types
    fn assert_single_type_matchup(
        attacker: TypeGen1,
        defender: TypeGen1,
        expected: f64,
        fast_impl: bool
    ) {
        let actual_result = if fast_impl {
            type_effectiveness_gen_1_fast(attacker, &[defender, TypeGen1::None])
        } else {
            type_effectiveness_gen_1(attacker, &[defender, TypeGen1::None])
        };
        
        assert!(
            (actual_result - expected).abs() < f64::EPSILON,
            "{:?} -> {:?}: expected {}, got {} (implementation: {})",
            attacker,
            defender,
            expected,
            actual_result,
            if fast_impl { "fast" } else { "original" }
        );
    }

    /// Test all single type matchups for a generation
    fn test_generation_effectiveness<F>(get_expected: F)
    where
        F: Fn(TypeGen1, TypeGen1) -> f64
    {
        for attacker in TypeGen1::iter() {
            for defender in TypeGen1::iter() {
                let expected = get_expected(attacker, defender);
                // Test both implementations
                assert_single_type_matchup(attacker, defender, expected, false);
                assert_single_type_matchup(attacker, defender, expected, true);
            }
        }
    }

    /// Test specific dual-type combinations
    fn test_dual_type_combinations(cases: &[(TypeGen1, [TypeGen1; 2], f64)]) {
        for case in cases {
            let (atk, defs, expected) = *case;
            let [def1, def2] = defs;
            
            let actual_slow = type_effectiveness_gen_1(atk, &defs);
            let actual_fast = type_effectiveness_gen_1_fast(atk, &defs);
            
            assert!(
                (actual_slow - expected).abs() < f64::EPSILON,
                "Dual-type {:?} -> {:?}/{:?}: expected {}, got {} (slow)",
                atk, def1, def2, expected, actual_slow
            );
            
            assert!(
                (actual_fast - expected).abs() < f64::EPSILON,
                "Dual-type {:?} -> {:?}/{:?}: expected {}, got {} (fast)",
                atk, def1, def2, expected, actual_fast
            );
        }
    }

    #[test]
    fn test_gen1_type_effectiveness() {
        test_generation_effectiveness(|attacker, defender| {
            let move_idx = attacker as usize;
            let def_idx = defender as usize;
            
            if (IMMUNE_MASK[move_idx] & (1 << def_idx)) != 0 {
                0.0
            } else if (SUPER_EFFECTIVE_MASK[move_idx] & (1 << def_idx)) != 0 {
                2.0
            } else if (NOT_VERY_EFFECTIVE_MASK[move_idx] & (1 << def_idx)) != 0 {
                0.5
            } else {
                1.0
            }
        });
    }

    #[test]
    fn test_gen1_dual_type_combinations() {
        let cases = [
            (TypeGen1::Electric, [TypeGen1::Ground, TypeGen1::Flying], 0.0),
            (TypeGen1::Grass, [TypeGen1::Water, TypeGen1::Ground], 4.0),
            (TypeGen1::Ghost, [TypeGen1::Normal, TypeGen1::Psychic], 0.0),
        ];
        
        test_dual_type_combinations(&cases);
    }
}