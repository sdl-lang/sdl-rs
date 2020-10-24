pub trait Renderable {
    fn render(&self) -> String;
    fn render_pretty(&self) -> String {
        self.render()
    }
}
