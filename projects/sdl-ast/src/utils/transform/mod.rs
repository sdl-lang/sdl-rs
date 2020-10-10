use crate::{ASTKind, AST};

impl AST {
    pub fn as_string(&self) -> String {
        match &self.kind {
            ASTKind::String(s) => format!("{}",s),
            ASTKind::Symbol(s) => format!("{:?}",s),
            ASTKind::Operation(s) => format!("{:?}",s),
            _ => String::new(),
        }
    }
}
