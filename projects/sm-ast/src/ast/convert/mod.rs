use super::*;

impl Default for ASTKind {
    fn default() -> Self {
        Self::Null
    }
}

impl<T> From<T> for ASTNode
where
    ASTKind: From<T>,
{
    fn from(v: T) -> Self {
        Self { kind: ASTKind::from(v), range: Default::default() }
    }
}

impl From<char> for ASTKind {
    fn from(v: char) -> Self {
        Self::String(v.to_string())
    }
}

impl From<&str> for ASTKind {
    fn from(v: &str) -> Self {
        Self::String(v.to_string())
    }
}

impl From<String> for ASTKind {
    fn from(v: String) -> Self {
        Self::String(v)
    }
}

impl From<&String> for ASTKind {
    fn from(v: &String) -> Self {
        Self::String(v.to_string())
    }
}

impl From<bool> for ASTKind {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}
