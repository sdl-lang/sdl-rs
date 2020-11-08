use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct HTMLElement {
    pub is_void: bool,
    pub tag: String,
    pub id: Vec<String>,
    pub class: BTreeSet<String>,
    pub attributes: BTreeSet<String>,
    pub arguments: BTreeMap<String, Value>,
    pub children: Vec<Value>,
}
