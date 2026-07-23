use crate::node::Node;
use crate::element::Element;

pub struct Seg2<const D: usize> {
    nodes: [Node<D>; 2]
}

impl<const D: usize> Seg2<D>  {
    pub fn new(n1: Node<D>, n2: Node<D>) -> Self {
        Seg2 {nodes: [n1, n2]}
    }
}

impl<const D:usize> Element<1,2> for Seg2<D> {
    fn id(&self) -> [usize; 2] {
        [self.nodes[0].id, self.nodes[1].id]
    }
    fn shape(&self, parametric_coordinates: [f64; 1]) -> [f64; 2] {
        let ξ = parametric_coordinates[0];
        [0.5*(1.0-ξ), 0.5*(1.0+ξ)]
    }
    fn jacobe(&self, _parametric_coordinates: [f64; 1]) -> f64 {
        (0..D).map(|i|(self.nodes[0].coordinates[i]-self.nodes[1].coordinates[i]).powi(2)).sum::<f64>().sqrt()*0.5
    }
    fn derivative_shape(&self, parametric_coordinates: [f64; 1]) -> [f64; 2] {
        let l = 2.0*self.jacobe(parametric_coordinates);
        [-1.0/l, 1.0/l]
    }
}