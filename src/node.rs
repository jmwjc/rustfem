use std::cmp::Eq;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::variable::Variable;

#[derive(Debug, Copy, Clone)]
pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}


pub struct BoundaryCondition {
    pub node: Node,
    pub variable: Variable,
    pub value: f64,
}