
pub mod gauss_segment;

pub trait Quadrature<const P: usize, const G: usize> {
    fn coordinates() -> [[f64; P]; G];
    fn weights() -> [f64; G];
}