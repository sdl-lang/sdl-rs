use super::*;

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

        let html = HTMLElement {
            is_void,
            tag,
            id: vec![],
            class: vec![],
            attributes: Default::default(),
            arguments: Default::default(),
            children: vec![],
        };
        Ok(Value::HTMLElement(Box::new(html)))
    }
}

impl Evaluate for Symbol {
    fn evaluate(&self, ctx: &mut SDLContext) -> Result<Value> {
        unimplemented!()
        // Ok(ASTKind::String(self.name()))
    }
}
