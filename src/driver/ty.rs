use core::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevType {
    Char,
    Block,
}

impl Display for DevType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DevType::Char => write!(f, "char"),
            DevType::Block => write!(f, "block"),
        }
    }
}
