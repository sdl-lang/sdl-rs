use crate::{ASTKind, AST};

impl AST {
    pub fn as_string(&self) -> String {
        match self.kind() {
            ASTKind::String(s) => s,
            _ => String::new(),
        }
    }
}
