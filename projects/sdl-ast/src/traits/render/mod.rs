use crate::{compile::HTMLElement, Result, SDLContext, Value};
use std::fmt::Write;

pub trait Render {
    fn render(&self, text: &mut impl Write, ctx: &SDLContext) -> Result<()>;
    fn render_pretty(&self, text: &mut impl Write, ctx: &SDLContext) -> Result<()> {
        self.render(text, ctx)
    }
}

impl Render for Value {
    fn render(&self, text: &mut impl Write, ctx: &SDLContext) -> Result<()> {
        match self {
            Value::Block(v) => {
                for e in v {
                    e.render(text, ctx)?
                }
            }
            Value::Null => write!(text, "null")?,
            Value::Boolean(v) => write!(text, "{}", v)?,
            Value::Integer(v) => write!(text, "{}", v)?,
            Value::String(v) => write!(text, "{:?}", v)?,
            Value::List(v) => {
                write!(text, "[")?;
                for (i, e) in v.iter().enumerate() {
                    e.render(text, ctx)?;
                    if i != v.len() - 1 {
                        write!(text, ", ")?;
                    }
                }
                write!(text, "]")?;
            }
            Value::Dict(v) => write!(text, "{:#?}", v)?,
            Value::HTMLElement(html) => html.render(text, ctx)?,
        };
        Ok(())
    }
    fn render_pretty(&self, text: &mut impl Write, ctx: &SDLContext) -> Result<()> {
        unimplemented!()
    }
}

impl Render for HTMLElement {
    fn render(&self, text: &mut impl Write, ctx: &SDLContext) -> Result<()> {
        let out = match self.is_void {
            true => format!("<{tag}>", tag = self.tag),
            false => format!("<{tag}></{tag}>", tag = self.tag),
        };
        write!(text, "{}", out)?;
        Ok(())
    }
}
