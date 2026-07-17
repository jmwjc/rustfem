pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Node {
    pub fn new(id: usize, x: f64, y: f64, z: f64) -> Self {
        Node {
            id: id,
            x: x,
            y: y,
            z: z,
        }
    }
}

pub trait Approximation<const DIM: usize, const NDOF: usize> {
    fn id(&self) -> [usize; NDOF];
    // fn shape(&self, x: [f64; DIM]) -> [f64; NDOF];
    // fn derivative_shape(&self, x: [f64; DIM]) -> [f64; NDOF];
    // fn jacobe(&self, x: [f64; DIM], coordinates: [Node; NDOF]) -> ([[f64; NDOF]; NDOF], f64); 
}
