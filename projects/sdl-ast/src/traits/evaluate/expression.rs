use super::*;

impl Evaluate for InfixExpression {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        let result = match self.op.as_string().as_str() {
            "+" => match (&self.lhs.kind, &self.rhs.kind) {
                (ASTKind::String(lhs), ASTKind::String(rhs)) => Value::String(String::from(lhs) + rhs),
                _ => unimplemented!("(ASTKind::{:?}, ASTKind::{:?}) => {{}}", &self.lhs.kind, &self.rhs.kind),
            },
            "==" | "is" => Value::Boolean(self.lhs.kind == self.rhs.kind),
            "!=" | "isnot" => Value::Boolean(self.lhs.kind != self.rhs.kind),
            _ => unimplemented!("Operation: {}", self.op.as_string().as_str()),
        };
        Ok(result)
    }
}

