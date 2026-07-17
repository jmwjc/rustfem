use crate::approximation::{Approximation, Node};
use crate::element::Seg2;

impl Seg2 {
    pub fn new(n1: Node, n2: Node) -> Self {
        Seg2 {
            dofs: [n1, n2],
        }
    }
}

impl Approximation<1, 2> for Seg2 {
    fn id(&self) -> [usize; 2] { std::array::from_fn(|i| self.dofs[i].id)}
    // fn shape(&self, x: [f64; 1]) -> [f64; 2] {
    //     let ξ = x[0];
    //     [0.5 * (1.0 - ξ), 0.5 * (1.0 + ξ)]
    // }
    // fn derivative_shape(&self, x: [f64; 1]) -> [f64; 2]{
    //     let ∂N₁∂ξ = 
    // }
}
