use super::*;
use crate::ast::ForInLoop;

impl AST {
    pub fn evaluate(&self, ctx: &mut Context) -> AST {
        let kind = self.kind.evaluate(ctx);
        AST { kind, range: self.range.to_owned() }
    }
}

impl ASTKind {
    pub fn evaluate(&self, ctx: &mut Context) -> ASTKind {
        match self {
            ASTKind::Program(v) => ASTKind::Program(v.iter().map(|e| e.evaluate(ctx)).collect()),
            ASTKind::Statement(v) => ASTKind::Statement(v.iter().map(|e| e.evaluate(ctx)).collect()),
            ASTKind::ForInLoop(inner) => inner.evaluate(ctx),
            ASTKind::Null | ASTKind::Boolean { .. } | ASTKind::String { .. } => self.to_owned(),
            _ => unimplemented!("ASTKind::{:?} => {{}}", self),
        }
    }
}

impl ForInLoop {
    pub fn evaluate(&self, ctx: &mut Context) -> ASTKind {
        unimplemented!("{:#?}", self);
    }
}
