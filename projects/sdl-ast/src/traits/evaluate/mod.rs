use crate::{ast::*, Result, RuntimeError, SDLContext};

mod ast;
mod control;
mod expression;
mod primitive;

pub trait Evaluate {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<ASTNode>;
}

pub trait Concat<Rhs = Self> {
    type Output;
    fn concat(self, rhs: Rhs) -> Self::Output;
}