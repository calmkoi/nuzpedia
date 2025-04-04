#[derive(Debug, Clone)]
pub struct Move {
    pub name: String,
    pub typ: Type,
    pub power: u8,
    pub category: MoveCategory,
}

#[derive(Debug, Clone)]
pub enum MoveCategory {
    Physical,
    Special,
    Status,
}