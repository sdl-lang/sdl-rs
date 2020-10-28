use crate::{compile::HTMLElement, Result, SDLContext, Value};
use std::{borrow::Cow, fmt::Write};

pub trait Render {
    fn render(&self, ctx: &mut SDLContext) -> Result<Cow<str>>;
    fn render_pretty(&self, ctx: &mut SDLContext) -> Result<Cow<str>> {
        self.render(ctx)
    }
}

impl Render for Value {
    fn render(&self, ctx: &mut SDLContext) -> Result<Cow<str>> {
        self.render_pretty(ctx)
    }

    fn render_pretty(&self, ctx: &mut SDLContext) -> Result<Cow<str>> {
        let result = match self {
            Value::Block(v) => {
                let mut out = String::new();
                for e in v {
                    write!(out, "{}", e.render(ctx)?)?
                }
                Cow::from(out)
            }
            Value::Null => Cow::from(String::new()),
            Value::Boolean(v) => Cow::from(v.to_string()),
            Value::String(v) => Cow::from(v),
            Value::List(v) => Cow::from(format!("{:#?}", v)),
            Value::Dict(v) => Cow::from(format!("{:#?}", v)),
            Value::HTMLElement(html) => html.render(ctx)?,
        };
        Ok(result)
    }
}

impl Render for HTMLElement {
    fn render(&self, ctx: &mut SDLContext) -> Result<Cow<str>> {
        let out = match self.is_void {
            true => format!("<{tag}>", tag = self.tag),
            false => format!("<{tag}></{tag}>", tag = self.tag),
        };
        Ok(Cow::from(out))
    }

    fn render_pretty(&self, ctx: &mut SDLContext) -> Result<Cow<str>> {
        unimplemented!()
    }
}
