use crate::{ast::*, compile::HTMLElement, Result, RuntimeError, SDLContext, Value};

mod ast;
mod control;
mod expression;
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
