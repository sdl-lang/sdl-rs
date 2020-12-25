use super::*;

impl Evaluate for InfixExpression {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        match self.op.as_string().as_str() {
            "+" => self.lhs.evaluate(ctx)? + self.rhs.evaluate(ctx)?,
            "-" => self.lhs.evaluate(ctx)? - self.rhs.evaluate(ctx)?,
            "*" => self.lhs.evaluate(ctx)? * self.rhs.evaluate(ctx)?,
            "/" => self.lhs.evaluate(ctx)? / self.rhs.evaluate(ctx)?,
            "==" | "is" => Ok(Value::Boolean(self.lhs.kind == self.rhs.kind)),
            "!=" | "isnot" => Ok(Value::Boolean(self.lhs.kind != self.rhs.kind)),
            _ => unimplemented!("Operation: {}", self.op.as_string().as_str()),
        }
    }
}

impl Evaluate for CallChain {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        let mut base = self.base.evaluate(ctx)?;
        for i in &self.chain {
            base = match &i.kind {
                ASTKind::CallIndex(n) => base.get_index(n.as_ref())?,
                _ => unimplemented!("ASTKind::{:?} => {{}}", i.kind),
            }
        }
        Ok(base)
    }
}
