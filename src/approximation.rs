struct Node {
    id: usize,
    x: f64,
    y: f64,
    z: f64,
}

pub trait Approximation<const DIM: usize, const NDOF: usize> {
    fn id(&self) -> [usize; NDOF];
    fn shape(&self, x: [f64; DIM]) -> [f64; NDOF];
    fn derivative_shape(&self, x: [f64; DIM]) -> [f64; NDOF];
    fn jacobe(&self, x: [f64; DIM], coordinates: [Node; NDOF]) -> ([[f64; NDOF]; NDOF], f64); 
}
