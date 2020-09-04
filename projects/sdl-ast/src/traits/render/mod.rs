use crate::{Template, AST};

pub trait Renderable {
    fn render(&self) -> String;
    fn render_pretty(&self) -> String {
        self.render()
    }
}

impl Renderable for AST {
    fn render(&self) -> String {
        String::new()
    }
}

impl Renderable for Template {
    fn render(&self) -> String {
        String::new()
    }
}
