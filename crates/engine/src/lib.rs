//! Pokémon battle mechanics library.
//!
//! Provides complete implementations for:
//! - Pokémon data structures
//! - Move calculations
//! - Damage formulas
//! - Type effectiveness
//!
//! # Example
//! ```
//! use poke_engine_gen1::{PokemonGen1, MoveGen1, calc_damage_gen_1, DamageRoll};
//!
//! let pikachu = PokemonGen1::new("Pikachu", [TypeGen1::Electric, TypeGen1::None]);
//! let thunderbolt = MoveGen1::new("Thunderbolt", TypeGen1::Electric, 90, MoveCategory::Special);
//! let damage = calc_damage_gen_1(&pikachu, &pikachu, &thunderbolt, false, DamageRoll::Average);
//! ```

pub mod pokemon;
pub mod poke_move;
pub mod damage;
pub mod types;

/// Re-export core battle types for convenient access.
pub use pokemon::{PokemonGen1, StatsGen1, StatStagesGen1, StatusGen1};
pub use poke_move::{MoveGen1, MoveCategory};
pub use damage::{DamageRoll, calc_damage_gen_1};
pub use types::{TypeGen1, type_effectiveness_gen_1, type_effectiveness_gen_1_fast};
