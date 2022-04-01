use crate::{ast::*, Result, SDLError, SDLContext};

mod ast;
mod control;
mod expression;
mod primitive;

pub trait Evaluate {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<ASTNode>;
    fn evaluate_kind(&self, ctx: &mut SDLContext) -> Result<ASTKind> {
        Ok(self.evaluate(ctx)?.kind)
    }
}

pub trait Concat<Rhs = Self> {
    type Output;
    fn concat(self, rhs: Rhs) -> Self::Output;
}