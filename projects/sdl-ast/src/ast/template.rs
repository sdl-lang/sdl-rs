use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Template {
    kind: TemplateKind,
    tag: Option<AST>,
    tag_end: Option<AST>,
    id: Option<AST>,
    class: Option<AST>,
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
        Self {
            kind: TemplateKind::OpenCloseTemplate,
            tag: None,
            tag_end: None,
            id: None,
            class: None
        }
    }
}


impl Template {
    pub fn open_close(start: RangedString, end: RangedString) -> Self {
        Self {
            kind: TemplateKind::OpenCloseTemplate,
            tag:None,
            tag_end:None,
            ..Self::default()
        }
    }
    pub fn self_close(start: RangedString) -> Self {
        Self {
            kind: TemplateKind::SelfCloseTemplate,
            tag:None,
            tag_end:None,
            ..Self::default()
        }
    }
    pub fn html_bad(start: RangedString) -> Self {
        Self {
            kind: TemplateKind::HTMLBadTemplate,
            tag:None,
            tag_end:None,
            ..Self::default()
        }
    }
    pub fn sdl_special(start: RangedString, end: RangedString) -> Self {
        Self {
            kind: TemplateKind::SDLSpecialTemplate,
            tag:None,
            tag_end:None,
            ..Self::default()
        }
    }
}

impl Template {

    pub fn set_tag(&mut self) {

    }
    pub fn set_tag_pair(&mut self) {

    }
    pub fn set_class(&mut self) {

    }
    pub fn set_id(&mut self) {

    }
}