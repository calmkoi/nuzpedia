use crate::TypeGen1;

// Generation 1 ----------------------------------------------------------
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

impl Default for MoveGen1 {
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
    fn default() -> Self {
        MoveCategory::Status
    }
}