use crate::{ast::*, Result, RuntimeError, SDLContext};

mod ast;
mod control;
mod expression;
mod primitive;

pub trait Evaluate {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<ASTNode>;
}
