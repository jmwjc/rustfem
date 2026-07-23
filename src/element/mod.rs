
pub mod poi1;
pub mod seg2;

pub trait Element<const P: usize, const N: usize> {
    fn id(&self) -> [usize; N];
    fn shape(&self, parametric_coordinates: [f64; P]) -> [f64; N]; 
    fn derivative_shape(&self, parametric_coordinates: [f64; P]) -> [f64; N]; 
    fn jacobe(&self, parametric_coordinates: [f64; P]) -> f64;
}