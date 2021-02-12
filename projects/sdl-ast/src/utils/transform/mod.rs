use crate::{ast::Symbol, ASTKind, ASTNode};

impl ASTNode {
    pub fn as_string(&self) -> String {
        match &self.kind {
            ASTKind::EscapedText(s) => format!("{}", s),
            ASTKind::Symbol(s) => format!("{:?}", s),
            ASTKind::Operator(s) => format!("{:?}", s),
            _ => String::new(),
        }
    }
    pub fn as_symbol(self) -> ASTNode {
        match &self.kind {
            ASTKind::EscapedText(_) => {
                let range = self.range.to_owned();
                ASTNode { kind: ASTKind::Symbol(Box::new(Symbol { path: vec![self] })), range }
            }
            ASTKind::Symbol(_) => self,
            _ => unreachable!(),
        }
    }
}
