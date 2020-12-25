use crate::{ast::Symbol, ASTKind, AST};

impl AST {
    pub fn as_string(&self) -> String {
        match &self.kind {
            ASTKind::EscapedText(s) => format!("{}", s),
            ASTKind::Symbol(s) => format!("{:?}", s),
            ASTKind::Operator(s) => format!("{:?}", s),
            _ => String::new(),
        }
    }
    pub fn as_symbol(self) -> AST {
        match &self.kind {
            ASTKind::EscapedText(_) => {
                let range = self.range.to_owned();
                AST { kind: ASTKind::Symbol(Box::new(Symbol { path: vec![self] })), range }
            }
            ASTKind::Symbol(_) => self,
            _ => unreachable!(),
        }
    }
}
