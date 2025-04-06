pub mod pokemon;
pub mod poke_move;
pub mod damage;
pub mod types;

// Re-export for easy external use
pub use pokemon::{PokemonGen1, StatsGen1, StatStagesGen1, StatusGen1};
pub use poke_move::{MoveGen1, MoveCategory};
pub use damage::calc_damage_gen_1;
pub use types::{TypeGen1, type_effectiveness_gen_1, type_effectiveness_gen_1_fast};
