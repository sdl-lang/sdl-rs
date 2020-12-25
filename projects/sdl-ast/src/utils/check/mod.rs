use crate::ASTKind;

impl ASTKind {
    pub fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }
    pub fn is_bool(&self) -> bool {
        match self {
            Self::Boolean { .. } => true,
            _ => false,
        }
    }
    pub fn is_true(&self) -> bool {
        match self {
            Self::Boolean(true) => true,
            _ => false,
        }
    }
    pub fn is_false(&self) -> bool {
        match self {
            Self::Boolean(false) => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            Self::EscapedText { .. } => true,
            Self::UnescapedText { .. } => true,
            _ => false,
        }
    }
}
