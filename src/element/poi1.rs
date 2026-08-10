use crate::node::Node;

pub struct Poi1<const D: usize> {
    pub nodes: Node<D>,
}

impl<const D: usize> Poi1<D> {
    pub fn new(node: Node<D>) -> Self {
        Poi1 { nodes: node }
    }
}
