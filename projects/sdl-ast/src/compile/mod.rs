mod value;
mod variable;

use crate::AST;
use std::{
    collections::HashMap,
    ops::Deref,
    rc::{Rc, Weak},
};
pub use value::Value;
pub use variable::Variable;

#[derive(Clone, Debug)]
pub struct Context {
    father: Option<Weak<Context>>,
    variables: HashMap<String, Variable>,
}

impl Default for Context {
    fn default() -> Self {
        Self { father: None, variables: Default::default() }
    }
}

impl Context {
    pub fn evaluate(&mut self, code: &AST) -> AST {
        code.to_owned()
    }
}

impl Context {
    pub fn get_value(&self, name: &str) -> Value {
        match self.variables.get(name) {
            Some(v) => v.get(),
            None => self.father.as_ref().and_then(|ctx| ctx.upgrade()).map(|ctx| ctx.get_value(name)).unwrap_or_default(),
        }
    }
}
