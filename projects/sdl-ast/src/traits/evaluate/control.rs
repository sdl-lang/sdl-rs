use super::*;

impl Evaluate for ForInLoop {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<ASTNode> {
        unimplemented!()
        // let terms = self.terms.evaluate(ctx)?;
        // let symbol = match &self.pattern.kind {
        //     ASTKind::Symbol(s) => s.name(),
        //     _ => unreachable!(),
        // };
        // let iter = match terms {
        //     Value::List(v) => v.into_iter(),
        //     Value::String(v) => {
        //         // FIXME: avoid collect
        //         let v: Vec<_> = v.chars().map(|e| Value::from(e)).collect();
        //         v.into_iter()
        //     }
        //     _ => unimplemented!("Value::{:?} => {{}}", terms),
        // };
        // let mut out = vec![];
        // for i in iter {
        //     let mut ctx = ctx.fork();
        //     ctx.insert(&symbol, i);
        //     let result = self.block.evaluate(&mut ctx)?;
        //     out.push(result)
        // }
        // Ok(Value::Block(out))
    }
}

impl Evaluate for IfElseChain {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<ASTNode> {
        unimplemented!()
        // for (cds, act) in &self.pairs {
        //     match cds.evaluate(ctx)? {
        //         Value::Boolean(true) => return Ok(act.evaluate(ctx)?),
        //         _ => continue,
        //     }
        // }
        // match &self.cover {
        //     Some(last) => Ok(last.evaluate(ctx)?),
        //     None => match ctx.config().is_debug {
        //         true => Err(RuntimeError::if_lost("TODO: ", format!(""))),
        //         false => Ok(Value::Null),
        //     },
        // }
    }
}
