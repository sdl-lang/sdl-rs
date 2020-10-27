use crate::{SDLContext};
use crate::Result;

pub trait Render {
    fn render(&self, ctx: &mut SDLContext) -> Result<String>;
    fn render_pretty(&self, ctx: &mut SDLContext) -> Result<String> {
        self.render(ctx)
    }
}
