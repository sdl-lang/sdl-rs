use super::*;


impl Evaluate for AST {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        self.kind.evaluate(ctx)
    }
}

impl Evaluate for ASTKind {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        let result = match self {
            ASTKind::Program(v) => Value::List(evaluate_vec_ast(v, ctx)?),
            ASTKind::Statement(v) => Value::List(evaluate_vec_ast(v, ctx)?),
            ASTKind::Expression(e, eos) => {
                let out = e.kind.evaluate(ctx)?;
                match eos {
                    true => Value::Null,
                    false => out,
                }
            }
            ASTKind::InfixExpression(inner) => inner.evaluate(ctx)?,

            ASTKind::IfElseChain(inner) => inner.evaluate(ctx)?,
            ASTKind::ForInLoop(inner) => inner.evaluate(ctx)?,
            ASTKind::Template(inner) => inner.evaluate(ctx)?,
            ASTKind::Symbol(inner) => inner.evaluate(ctx)?,
            ASTKind::Null => Value::Null,
            ASTKind::Boolean(v) => Value::Boolean(v.to_owned()),
            ASTKind::String(v) => Value::String(v.to_owned()),
            _ => unimplemented!("ASTKind::{:?} => {{}}", self),
        };
        Ok(result)
    }
}

