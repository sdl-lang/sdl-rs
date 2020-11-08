use super::*;
use std::collections::BTreeSet;

#[rustfmt::skip]
pub static VOID_TAGS: &[&str; 16] = &[
    "img", "hr", "br", "input", "link", "meta", "area", "base", "col", "wbr",
    "command", "embed", "keygen", "param", "source", "track"
];

impl Evaluate for Template {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        let tag = match &self.tag {
            None => unimplemented!(),
            Some(s) => s.as_string(),
        };
        let is_void = VOID_TAGS.contains(&tag.as_str());

        let mut class = BTreeSet::new();

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

        let html =
            HTMLElement { is_void, tag, id: vec![], class, attributes: Default::default(), arguments: Default::default(), children: vec![] };
        Ok(Value::HTMLElement(Box::new(html)))
    }
}

impl Evaluate for Symbol {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        Ok(ctx.get(&self.name()))
    }
}
