use crate::{SDLContext, Value};
use crate::Result;
use crate::{ast::*};
use crate::RuntimeError;

mod expression;
mod ast;
mod control;
mod primitive;

pub trait Evaluate {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value>;
}

fn evaluate_vec_ast(v: &[AST], ctx: &mut SDLContext) -> Result<Vec<Value>> {
    let mut collected = Vec::with_capacity(v.len());
    for e in v {
        let out = e.evaluate(ctx)?;
        if !out.is_null() {
            collected.push(out)
        }
    }
    Ok(collected)
}
