use super::*;


impl Evaluate for Template {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        unimplemented!()
       // let norm = self.regularized();
      //  Ok(ASTKind::TemplateSimplified(Box::from(norm)))
    }
}

impl Evaluate for Symbol {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        unimplemented!()
        // Ok(ASTKind::String(self.name()))
    }
}


