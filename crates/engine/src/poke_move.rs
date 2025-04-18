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