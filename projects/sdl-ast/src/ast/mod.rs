use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};
use crate::TextRange;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AST {
    Node {
        kind: ASTKind,
        //      children: Box<[AST]>,
        children: Vec<AST>,
        r: Option<Box<TextRange>>,
    },
    Leaf {
        kind: ASTKind,
        r: Option<Box<TextRange>>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ASTKind {
    None,
    Statements,
    ///
    /// eg: <img> </img>
    OpenCloseTemplate,
    ///
    /// eg: <img/>
    SelfCloseTemplate,
    ///
    /// eg: <img>
    HTMLSpecialTemplate,
    ///
    /// <\img> </img>
    SDLSpecialTemplate

}

impl Default for AST {
    fn default() -> Self {
        Self::Leaf { kind: ASTKind::None, r: Default::default() }
    }
}

impl AST {
    pub fn kind(&self) -> ASTKind {
        match self {
            Self::Node { kind, .. } | Self::Leaf { kind, .. } => kind.to_owned(),
        }
    }

    pub fn children(&self) -> Vec<AST> {
        match self {
            Self::Node { children, .. } => children.to_vec(),
            Self::Leaf { .. } => vec![],
        }
    }
    pub fn range(&self) -> TextRange {
        match self {
            Self::Node { r, .. } | Self::Leaf { r, .. } => r.clone().unwrap_or_default().as_ref().clone(),
        }
    }
}

impl AST {
    pub fn statements(children: Vec<AST>, r: TextRange) -> Self {
        Self::Node { kind: ASTKind::Statements, children, r: box_range(r) }
    }
}

fn box_range(r: TextRange) -> Option<Box<TextRange>> {
    match r.sum() {
        0 => None,
        _ => box_range(r),
    }
}
