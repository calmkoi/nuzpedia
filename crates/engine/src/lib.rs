pub mod pokemon;
pub mod poke_move;
pub mod damage;

// Re-export for easy external use
pub use pokemon::Pokemon;
pub use poke_move::{Move, MoveCategory};
pub use damage::calculate_gen1;
