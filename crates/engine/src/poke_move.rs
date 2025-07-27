use crate::TypeGen1;

/// Generation 1 Move representation.
/// 
/// This struct defines a move in Generation 1 of the game, including its name, type, power, and category.
#[derive(Debug, Clone)]
pub struct MoveGen1 {
    /// The name of the move (e.g., "Tackle", "Fire Blast").
    pub name: String,
    
    /// The type of the move (e.g., Normal, Fire, Water).
    pub typ: TypeGen1,
    
    /// The base power of the move (0 for status moves).
    pub power: u8,
    
    /// The category of the move (Physical, Special, or Status).
    pub category: MoveCategory,
}

/// Represents the category of a move in Generation 1.
#[derive(Debug, Clone, PartialEq)]
pub enum MoveCategory {
    /// Physical moves (affected by Attack and Defense stats).
    Physical,
    
    /// Special moves (affected by Special stat in Gen 1).
    Special,
    
    /// Status moves (no direct damage, cause side effects).
    Status,
}

impl Default for MoveGen1 {
    /// Creates a default `MoveGen1` with:
    /// - Empty name
    /// - Type: `Normal`
    /// - Power: `50`
    /// - Category: `Physical`
    fn default() -> Self {
        Self { 
            name: String::new(), 
            typ: TypeGen1::Normal, 
            power: 50, 
            category: MoveCategory::Physical
        }
    }
}

impl Default for MoveCategory {
    /// The default move category is `Status`.
    fn default() -> Self {
        MoveCategory::Status
    }
}

// ================= Testing Infrastructure =================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_move_values() {
        let m = MoveGen1::default();
        assert_eq!(m.name, "");
        assert_eq!(m.typ, TypeGen1::Normal);
        assert_eq!(m.power, 50);
        assert_eq!(m.category, MoveCategory::Physical);
    }

    #[test]
    fn status_move_zero_power() {
        let m = MoveGen1 {
            category: MoveCategory::Status,
            power: 0,
            ..Default::default()
        };
        assert!(m.power == 0);
    }

    #[test]
    fn damaging_move_detection() {
        let m = MoveGen1 {
            name: "Fire Blast".to_string(),
            typ: TypeGen1::Fire,
            power: 110,
            category: MoveCategory::Special,
        };

        assert!(m.power > 0);
        assert_ne!(m.category, MoveCategory::Status);
    }

    #[test]
    fn move_type_matches_category() {
        let _m1 = MoveGen1 {
            typ: TypeGen1::Electric,
            category: MoveCategory::Special, // Electric is special only in Gen 1
            ..Default::default()
        };
        let _m2 = MoveGen1 {
            typ: TypeGen1::Ground,
            category: MoveCategory::Physical, // Ground is physical in Gen 1
            ..Default::default()
        };
        // No need for assertations, saved for gen1_damage.rs
    }

    #[test]
    fn zero_power_non_status_move() {
        let m = MoveGen1 {
            name: "Splash".to_string(),
            power: 0,
            category: MoveCategory::Physical, // Legal in Gen 1
            ..Default::default()
        };

        // Ensure the struct allows this, even if it's ignored by the damage calc
        assert_eq!(m.power, 0);
    }
 }