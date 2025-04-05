pub mod pokemon;
pub mod poke_move;
pub mod damage;

// Re-export for easy external use
pub use pokemon::{PokemonGen1, TypeGen1, StatsGen1};
pub use poke_move::{MoveGen1, MoveCategory};
pub use damage::calc_damage_gen_1;
