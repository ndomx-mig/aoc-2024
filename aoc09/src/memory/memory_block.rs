use std::fmt::Display;

use super::memory_type::MemoryType;

#[derive(Debug)]
pub struct MemoryBlock {
    pub memory_type: MemoryType,
    pub id: Option<u32>,
}

impl Display for MemoryBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return if let Some(id) = self.id {
            write!(f, "{}", id)
        } else {
            write!(f, "{}", '.')
        };
    }
}
