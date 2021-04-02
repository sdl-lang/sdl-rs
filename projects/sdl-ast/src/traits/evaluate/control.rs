use super::*;
use crate::utils::get_variant_name;

impl Evaluate for ForInLoop {
    fn evaluate(&self, _: &mut SDLContext) -> Result<ASTNode> {
        unreachable!()
    }

    fn evaluate_kind(&self, ctx: &mut SDLContext) -> Result<ASTKind> {
        let terms = self.terms.evaluate(ctx)?;
        let symbol = match &self.pattern.kind {
            ASTKind::Symbol(s) => s.name(),
            _ => unreachable!(),
        };
        let items = match terms.kind {
            ASTKind::List(v) => v,
            ASTKind::String(v) => {
                // FIXME: avoid collect
                v.chars().map(|e| ASTNode::from(e)).collect()
            }
            _ => return Err(SDLError::invalid_iterator( get_variant_name(terms.kind),terms.range)),
        };
        let mut out = vec![];
        for i in items {
            let mut ctx = ctx.fork();
            ctx.insert(&symbol, i);
            let result = self.block.evaluate(&mut ctx)?;
            out.push(result)
        }
        Ok(ASTKind::Block(out))
    }
}

impl Evaluate for IfElseChain {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<ASTNode> {
        for (cds, act) in &self.pairs {
            match cds.evaluate(ctx)?.kind {
                ASTKind::Boolean(true) => return Ok(act.evaluate(ctx)?),
                _ => continue,
            }
        }
        match &self.cover {
            Some(last) => Ok(last.evaluate(ctx)?),
            None => {
                let err = SDLError::if_lost(self.range);
                match ctx.config().is_debug {
                    true => Err(err),
                    false => {
                        println!("{}", err);
                        Ok(ASTNode {
                            kind: ASTKind::Null,
                            range: self.range
                        })
                    },
                }
            }
        }
    }
}
