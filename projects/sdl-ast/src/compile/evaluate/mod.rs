use super::*;
use crate::{
    ast::{ForInLoop, IfElseChain, InfixExpression, Symbol},
    Template,
};

impl AST {
    pub fn evaluate(&self, ctx: &mut Context) -> Result<AST> {
        let kind = self.kind.evaluate(ctx)?;
        Ok(AST { kind, range: self.range.to_owned() })
    }
}

impl ASTKind {
    pub fn evaluate(&self, ctx: &mut Context) -> Result<ASTKind> {
        let result = match self {
            ASTKind::Program(v) => ASTKind::Program(evaluate_vec_ast(v, ctx)?),
            ASTKind::Statement(v) => ASTKind::Statement(evaluate_vec_ast(v, ctx)?),
            ASTKind::Expression(e, eos) => {
                let out = e.kind.evaluate(ctx)?;
                match eos {
                    true => ASTKind::Null,
                    false => out,
                }
            }
            ASTKind::InfixExpression(inner) => inner.evaluate(ctx)?,

            ASTKind::IfElseChain(inner) => inner.evaluate(ctx)?,
            ASTKind::ForInLoop(inner) => inner.evaluate(ctx)?,
            ASTKind::Template(inner) => inner.evaluate(ctx)?,
            ASTKind::Symbol(inner) => inner.evaluate(ctx)?,
            ASTKind::Null | ASTKind::Boolean { .. } | ASTKind::String { .. } => self.to_owned(),

            _ => unimplemented!("ASTKind::{:?} => {{}}", self),
        };
        Ok(result)
    }
}

impl ForInLoop {
    pub fn evaluate(&self, ctx: &mut Context) -> Result<ASTKind> {
        match &self.pattern.kind {
            ASTKind::Symbol(s) => println!("{:#?}", s.name()),
            _ => unreachable!(),
        }

        unimplemented!("{:#?}", self);
    }
}

impl IfElseChain {
    pub fn evaluate(&self, ctx: &mut Context) -> Result<ASTKind> {
        for (cds, act) in &self.pairs {
            match cds.evaluate(ctx)?.kind {
                ASTKind::Boolean(true) => return Ok(act.evaluate(ctx)?.kind),
                _ => continue,
            }
        }
        match &self.cover {
            Some(last) => Ok(last.evaluate(ctx)?.kind),
            None => Ok(ASTKind::Null),
        }
    }
}

impl InfixExpression {
    pub fn evaluate(&self, ctx: &mut Context) -> Result<ASTKind> {
        let result = match self.op.as_string().as_str() {
            "+" => match (&self.lhs.kind, &self.rhs.kind) {
                (ASTKind::String(lhs), ASTKind::String(rhs)) => ASTKind::String(String::from(lhs) + rhs),
                _ => unimplemented!("(ASTKind::{:?}, ASTKind::{:?}) => {{}}", &self.lhs.kind, &self.rhs.kind),
            },
            "==" | "is" => ASTKind::Boolean(self.lhs.kind == self.rhs.kind),
            "!=" | "isnot" => ASTKind::Boolean(self.lhs.kind != self.rhs.kind),
            _ => unimplemented!("Operation: {}", self.op.as_string().as_str()),
        };
        Ok(result)
    }
}

impl Template {
    pub fn evaluate(&self, ctx: &mut Context) -> Result<ASTKind> {
        let norm = self.regularized();
        Ok(ASTKind::TemplateSimplified(Box::from(norm)))
    }
}

impl Symbol {
    pub fn evaluate(&self, ctx: &mut Context) -> Result<ASTKind> {
        Ok(ASTKind::String(self.name()))
    }
}

pub fn evaluate_vec_ast(v: &[AST], ctx: &mut Context) -> Result<Vec<AST>> {
    let mut collected = Vec::with_capacity(v.len());
    for e in v {
        let out = e.evaluate(ctx)?;
        if !out.kind.is_null() {
            collected.push(out)
        }
    }
    Ok(collected)
}
