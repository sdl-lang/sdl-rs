use super::*;
use crate::ast::{ForInLoop, InfixExpression};

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
            ASTKind::Expression(e, eos)=> {
                let out = e.kind.evaluate(ctx);
                match eos {
                    true => ASTKind::Null,
                    false => out
                }
            }
            ASTKind::InfixExpression(inner) => inner.evaluate(ctx),

            ASTKind::ForInLoop(inner) => inner.evaluate(ctx),
            ASTKind::Null | ASTKind::Boolean { .. } | ASTKind::String { .. } => self.to_owned(),
            _ => unimplemented!("ASTKind::{:?} => {{}}", self),
        }
    }
}

impl ForInLoop {
    pub fn evaluate(&self, ctx: &mut Context) -> ASTKind {
        match &self.pattern.kind {
            ASTKind::Symbol(s)=> println!("{:#?}", s.name()),
            _ => unreachable!()
        }

        unimplemented!("{:#?}", self);
    }
}

impl InfixExpression {
    pub fn evaluate(&self, ctx: &mut Context) -> ASTKind {
        match self.op.as_string().as_str() {
            "+" => match (&self.lhs.kind, &self.rhs.kind) {
                (ASTKind::String(lhs), ASTKind::String(rhs)) => {
                    ASTKind::String(String::from(lhs) + rhs)
                }


                _ => unimplemented!("(ASTKind::{:?}, ASTKind::{:?}) => {{}}", &self.lhs.kind, &self.rhs.kind)
            },
            _ => unimplemented!("Operation: {}", self.op.as_string().as_str())
        }
    }
}
