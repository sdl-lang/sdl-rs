use super::*;
use crate::{traits::Render};
use std::collections::BTreeSet;

#[rustfmt::skip]
pub static VOID_TAGS: &[&str; 16] = &[
    "img", "hr", "br", "input", "link", "meta", "area", "base", "col", "wbr",
    "command", "embed", "keygen", "param", "source", "track"
];

impl Evaluate for Template {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<ASTNode> {
        let tag = match &self.tag {
            None => unimplemented!(),
            Some(s) => s.as_string(),
        };
        let is_void = VOID_TAGS.contains(&tag.as_str());

        // let class = BTreeSet::new();

        // if let Some(s) = &self.class {
        //     match &s.kind {
        //         ASTKind::String(s) => {
        //             for i in s.split(" ") {
        //                 class.insert(i.to_string())
        //             }
        //         }
        //         _ => ()
        //     }
        // }
        unimplemented!()
        // let html =
        //     HTMLElement { is_void, tag, id: vec![], class, attributes: Default::default(), arguments: Default::default(), children: vec![] };
        // Ok(Value::HTMLElement(Box::new(html)))
    }
}

impl Evaluate for Symbol {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<ASTNode> {
        unimplemented!()
        // Ok(ctx.get(&self.name()))
    }
}

impl Evaluate for StringExpression {
    fn evaluate(&self, _: &mut SDLContext) -> Result<ASTNode> {
        unreachable!()
    }

    fn evaluate_kind(&self, ctx: &mut SDLContext) -> Result<ASTKind> {
        let out = match self.handler {
            Some(_) => unimplemented!(),
            None => {
                let mut out = String::new();
                for e in &self.inner {
                    e.evaluate(ctx)?.render(&mut out, ctx)?;
                }
                ASTKind::String(out)
            }
        };
        Ok(out)
    }
}
