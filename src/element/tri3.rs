use crate::node::Node;

pub struct Tri3<const D: usize> {
    nodes: [Node<D>; 3],
}

impl<const D: usize> Tri3<D> {
    pub fn new(n1: Node<D>, n2: Node<D>, n3: Node<D>) -> Self {
        Tri3 {
            nodes: [n1, n2, n3],
        }
    }

    pub fn id(&self) -> [usize; 3] {
        [self.nodes[0].id, self.nodes[1].id, self.nodes[2].id]
    }
}
