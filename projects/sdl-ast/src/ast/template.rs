use super::*;

#[derive(Clone, Eq, PartialEq)]
pub struct Template {
    kind: TemplateKind,
    tag: Option<AST>,
    end: Option<AST>,
    id: Option<AST>,
    class: Option<AST>,
    attributes: Vec<AST>,
    arguments: Vec<AST>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TemplateKind {
    /// eg: <img> </img>
    OpenCloseTemplate,
    /// eg: <img/>
    SelfCloseTemplate,
    /// eg: <img>
    HTMLBadTemplate,
    /// <\img> </img>
    SDLSpecialTemplate,
}

impl Default for Template {
    fn default() -> Self {
        Self { kind: TemplateKind::OpenCloseTemplate, tag: None, end: None, id: None, class: None, attributes: vec![], arguments: vec![] }
    }
}

impl Template {
    pub fn open_close(start: AST, end: AST) -> Self {
        assert!(start.kind().is_string());
        assert!(end.kind().is_string());
        Self { kind: TemplateKind::OpenCloseTemplate, tag: Some(start), end: Some(end), ..Self::default() }
    }
    pub fn self_close(start: AST) -> Self {
        assert!(start.kind().is_string());
        Self { kind: TemplateKind::SelfCloseTemplate, tag: Some(start), end: None, ..Self::default() }
    }
    pub fn html_bad(start: AST) -> Self {
        assert!(start.kind().is_string());
        Self { kind: TemplateKind::HTMLBadTemplate, tag: Some(start), end: None, ..Self::default() }
    }
    pub fn sdl_special(start: AST, end: AST) -> Self {
        assert!(start.kind().is_string());
        assert!(end.kind().is_string());
        Self { kind: TemplateKind::SDLSpecialTemplate, tag: Some(start), end: Some(end), ..Self::default() }
    }
}

impl Template {
    pub fn set_class(&mut self) {}
    pub fn set_id(&mut self) {}
    pub fn set_attributes(&mut self, values: Vec<AST>) {
        self.attributes = values
    }
    pub fn set_arguments(&mut self, values: Vec<AST>) {
        self.arguments = values
    }
}

impl Debug for Template {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.tag {
            None => write!(f, "<>"),
            Some(s) => {
                let mut out = f.debug_struct(&format!("<{}>",s.as_string()));
                if self.attributes.len() != 0 {
                    out.field("attributes", &self.attributes);
                }
                if self.arguments.len() != 0 {
                    out.field("arguments", &self.arguments);
                }
                out.finish()
            }
        }
    }
}
