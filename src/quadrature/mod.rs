
pub mod gauss_segment;

pub trait Quadrature<const P: usize, const G: usize> {
    fn coordinates(&self) -> [[f64; P]; G];
    fn weights(&self) -> [f64; G];
}