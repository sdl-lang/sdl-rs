use crate::ASTKind;

impl ASTKind {
    pub fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            Self::String { .. } => true,
            _ => false,
        }
    }
}
