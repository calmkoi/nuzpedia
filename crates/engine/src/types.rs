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
    // Check immunities
    if IMMUNE.contains(&(move_type, defender_types[0])) ||
       IMMUNE.contains(&(move_type, defender_types[1])) {
        return 0.0;
    }

    // Calculate multiplier for defender type
    let mut multiplier = 1.0;
    for &defender_type in defender_types {
        if defender_type == TypeGen1::None {
            continue; // skip placeholder type
        }
        if SUPER_EFFECTIVE.contains(&(move_type, defender_type)) {
            multiplier *= 2.0;
        } else if NOT_VERY_EFFECTIVE.contains(&(move_type, defender_type)) {
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
    
    #[test]
    fn test_type_effectiveness_gen_1() {
        // Water vs. Fire (2x)
        assert_eq!(type_effectiveness_gen_1(TypeGen1::Water, &[TypeGen1::Fire, TypeGen1::None]), 2.0);
        // Electric vs. Ground (0x)
        assert_eq!(type_effectiveness_gen_1(TypeGen1::Electric, &[TypeGen1::Ground, TypeGen1::None]), 0.0);
        // Grass vs. Water/Ground (4x)
        assert_eq!(type_effectiveness_gen_1(TypeGen1::Grass, &[TypeGen1::Water, TypeGen1::Ground]), 4.0);
    }
}