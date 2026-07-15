use crate::approximation::Approximation;
use crate::element::Seg2;

impl Seg2 {
    pub fn new(i1: usize, i2: usize) -> Self {
        Seg2 {
            vertices: [i1, i2],
        }
    }
}

impl Approximation<1, 2> for Seg2 {
    fn id(&self) -> [usize; 2] { self.vertices }
    fn shape(&self, x: [f64; 1]) -> [f64; 2] {
        let ξ = x[0];
        [0.5 * (1.0 - ξ), 0.5 * (1.0 + ξ)]
    }
    // fn derivative_shape(&self, x: [f64; 1]) -> [f64; 2]{
    //     let ∂N₁∂ξ = 
    // }
}
