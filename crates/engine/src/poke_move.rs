use crate::pokemon::Type;

#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub typ: Type,
    pub power: u8,
    pub category: MoveCategory,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}