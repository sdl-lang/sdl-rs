use super::*;


impl Evaluate for ASTNode {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<ASTNode> {
       let result = match &self.kind {
            ASTKind::Program(v)|ASTKind::Statement(v) => {
                ASTNode {
                    kind: ASTKind::Block(evaluate_vec_ast(v, ctx)?),
                    range: self.range
                }
            },
            ASTKind::Expression(e, eos) => {
                let out = e.kind.evaluate(ctx)?;
                ASTNode {
                    kind: match eos {
                        true => ASTKind::Null,
                        false => out.kind,
                    },
                    range: self.range
                }
            }
            ASTKind::InfixExpression(inner) => inner.evaluate(ctx)?,
            ASTKind::StringExpression(inner) => inner.evaluate(ctx)?,

            ASTKind::IfElseChain(inner) => inner.evaluate(ctx)?,
            ASTKind::ForInLoop(inner) => inner.evaluate(ctx)?,
            ASTKind::Template(inner) => inner.evaluate(ctx)?,
            ASTKind::Symbol(inner) => inner.evaluate(ctx)?,

            ASTKind::CallChain(inner) => inner.evaluate(ctx)?,

            ASTKind::List(inner) => {
                let list = inner.iter().flat_map(|e| e.evaluate(ctx)).collect();
                ASTNode {
                    kind: ASTKind::List(list),
                    range: self.range
                }
            },

            ASTKind::Null |
            ASTKind::Boolean(_) |
            ASTKind::HTMLText(_) |
            ASTKind::String(_) |
            ASTKind::Integer(_) |
            ASTKind::Decimal(_) => self.to_owned(),

            _ => unimplemented!("ASTKind::{:?} => {{}}", self),
        };
        Ok(result)
    }
}

impl Evaluate for ASTKind {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<ASTNode> {
        ASTNode {
            kind: self.to_owned(),
            range: Default::default()
        }.evaluate(ctx)
    }
}


fn evaluate_vec_ast(v: &[ASTNode], ctx: &mut SDLContext) -> Result<Vec<ASTNode>> {
    let mut collected = Vec::with_capacity(v.len());
    for e in v {
        let out = e.evaluate(ctx)?;
        if !out.kind.is_null() {
            collected.push(out)
        }
    }
    Ok(collected)
}
