use super::*;

impl Evaluate for ForInLoop {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        match &self.pattern.kind {
            ASTKind::Symbol(s) => println!("{:#?}", s.name()),
            _ => unreachable!(),
        }

        unimplemented!("{:#?}", self);
    }
}

impl Evaluate for IfElseChain {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        for (cds, act) in &self.pairs {
            match cds.evaluate(ctx)? {
                Value::Boolean(true) => return Ok(act.evaluate(ctx)?),
                _ => continue,
            }
        }
        match &self.cover {
            Some(last) => Ok(last.evaluate(ctx)?),
            None => match ctx.config().is_debug {
                true => Err(RuntimeError::if_lost("TODO: ")),
                false => Ok(Value::Null),
            },
        }
    }
}
