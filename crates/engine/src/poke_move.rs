use crate::TypeGen1;

#[derive(Debug, Clone)]
pub struct MoveGen1 {
    pub name: String,
    pub typ: TypeGen1,
    pub power: u8,
    pub category: MoveCategory,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}