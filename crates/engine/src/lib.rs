pub mod pokemon;
pub mod move;
pub mod damage;

// Re-export for easy external use
pub use pokemon::Pokemon;
pub use move::{Move, MoveCategory};
pub damage::calculate_gen1;
