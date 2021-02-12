use super::*;

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Template {
    pub kind: TemplateKind,
    pub tag: Option<ASTNode>,
    pub end: Option<ASTNode>,
    pub attributes: Vec<ASTNode>,
    pub arguments: Vec<(ASTNode, ASTNode)>,
    pub children: Vec<ASTNode>,
}

impl Default for Template {
    fn default() -> Self {
        Self { kind: TemplateKind::OpenCloseTemplate, tag: None, end: None, attributes: vec![], arguments: vec![], children: vec![] }
    }
}

impl Template {
    pub fn open_close() -> Self {
        Self { kind: TemplateKind::OpenCloseTemplate, ..Self::default() }
    }
    pub fn self_close() -> Self {
        Self { kind: TemplateKind::SelfCloseTemplate, ..Self::default() }
    }
    pub fn html_bad() -> Self {
        Self { kind: TemplateKind::HTMLBadTemplate, ..Self::default() }
    }
    pub fn sdl_special() -> Self {
        Self { kind: TemplateKind::SDLSpecialTemplate, ..Self::default() }
    }
}

impl Template {
    pub fn set_tag(&mut self, tag: ASTNode) {
        self.tag = Some(tag)
    }
    pub fn set_end(&mut self, tag: ASTNode) {
        self.end = Some(tag)
    }
    pub fn set_attributes(&mut self, values: Vec<ASTNode>) {
        self.attributes = values
    }
    pub fn set_arguments(&mut self, values: Vec<(ASTNode, ASTNode)>) {
        self.arguments = values
    }
}

/*
impl Debug for Template {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.tag {
            None => write!(f, "<>"),
            Some(s) => {
                let mut out = f.debug_struct(&format!("<{}>", s.as_string()));
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
*/
