use crate::{Result, SDLContext, ASTNode, ASTKind};
use std::fmt::Write;

pub trait Render {
    fn render(&self, text: &mut impl Write, ctx: &SDLContext) -> Result<()>;
    fn render_pretty(&self, text: &mut impl Write, ctx: &SDLContext) -> Result<()> {
        self.render(text, ctx)
    }
}

impl Render for ASTNode {
    fn render(&self, text: &mut impl Write, ctx: &SDLContext) -> Result<()> {
        self.kind.render(text, ctx)
    }
}

impl Render for ASTKind {
    fn render(&self, text: &mut impl Write, ctx: &SDLContext) -> Result<()> {
        match self {
            Self::None => (),
            Self::Block(v) => {
                for e in v {
                    e.render(text, ctx)?
                }
            }
            Self::Null => write!(text, "null")?,
            Self::Boolean(v) => write!(text, "{}", v)?,
            Self::Integer(v) => write!(text, "{}", v)?,
            Self::Decimal(v) => write!(text, "{}", v)?,
            Self::HTMLText(v) | Self::String(v) => write!(text, "{:?}", v)?,
            Self::List(v) => {
                write!(text, "[")?;
                for (i, e) in v.iter().enumerate() {
                    e.render(text, ctx)?;
                    if i != v.len() - 1 {
                        write!(text, ", ")?;
                    }
                }
                write!(text, "]")?;
            }
            _ => unimplemented!("{:?}", self)
            // Self::Dict(v) => write!(text, "{:#?}", v)?,
            // Self::HTMLElement(html) => html.render(text, ctx)?,
        };
        Ok(())
    }
    fn render_pretty(&self, text: &mut impl Write, ctx: &SDLContext) -> Result<()> {
        unimplemented!()
    }
}

// impl Render for HTMLElement {
//     fn render(&self, text: &mut impl Write, ctx: &SDLContext) -> Result<()> {
//         let out = match self.is_void {
//             true => format!("<{tag}>", tag = self.tag),
//             false => format!("<{tag}></{tag}>", tag = self.tag),
//         };
//         write!(text, "{}", out)?;
//         Ok(())
//     }
// }
