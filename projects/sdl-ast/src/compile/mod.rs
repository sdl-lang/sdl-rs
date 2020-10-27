#[allow(unused_variables)]
mod renderer;
mod value;
mod variable;

use crate::{ASTKind, Result, AST};
use std::{collections::HashMap, rc::Weak};
pub use value::Value;
pub use variable::Variable;
use crate::traits::Evaluate;

#[derive(Clone, Debug)]
pub struct SDLContext {
    config: Option<Box<SDLContextConfig>>,
    father: Option<Weak<SDLContext>>,
    variables: HashMap<String, Variable>,
}

#[derive(Clone, Debug)]
pub struct SDLContextConfig {
    pub is_debug: bool,
}

impl Default for SDLContext {
    fn default() -> Self {
        Self { config: None, father: None, variables: Default::default() }
    }
}

impl Default for SDLContextConfig {
    fn default() -> Self {
        Self { is_debug: false }
    }
}

impl SDLContext {
    pub fn evaluate(&mut self, code: &AST) -> Result<Value> {
        code.evaluate(self)
    }
    pub fn render(&mut self, code: &AST) -> Result<String> {
        code.render(self)
    }
    pub fn config(&self) -> SDLContextConfig {
        match &self.config {
            None => Default::default(),
            Some(x) => *x.clone(),
        }
    }
}

impl SDLContext {
    pub fn get_value(&self, name: &str) -> Value {
        match self.variables.get(name) {
            Some(v) => v.get(),
            None => self.father.as_ref().and_then(|ctx| ctx.upgrade()).map(|ctx| ctx.get_value(name)).unwrap_or_default(),
        }
    }
}
