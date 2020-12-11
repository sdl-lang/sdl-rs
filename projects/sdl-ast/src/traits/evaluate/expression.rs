use super::*;
use std::ops::Index;

impl Evaluate for InfixExpression {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        let result = match self.op.as_string().as_str() {
            "+" => match (self.lhs.evaluate(ctx)?, self.rhs.evaluate(ctx)?) {
                (Value::String(lhs), Value::String(rhs)) => Value::String(lhs + &rhs),
                _ => unimplemented!("(ASTKind::{:?}, ASTKind::{:?}) => {{}}", &self.lhs.kind, &self.rhs.kind),
            },
            "==" | "is" => Value::Boolean(self.lhs.kind == self.rhs.kind),
            "!=" | "isnot" => Value::Boolean(self.lhs.kind != self.rhs.kind),
            _ => unimplemented!("Operation: {}", self.op.as_string().as_str()),
        };
        Ok(result)
    }
}


impl Evaluate for CallChain {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        let mut base = self.base.evaluate(ctx)?;
        for i in &self.chain {
            base =  match &i.kind {
               ASTKind::CallIndex(n) => {
                    match base.index(n.as_ref()) {
                        Ok(o) => o.to_owned(),
                        Err(e) => {
                            return Err(e.clone())
                        }
                    }
                }
                _ => unimplemented!("ASTKind::{:?} => {{}}", i.kind)
            }
        }
        Ok(base)
    }
}