
pub mod poi1;
pub mod seg2;

pub trait Element<const D: usize, const P: usize, const N: usize> {
    fn id(&self) -> [usize; N];
    fn shape(&self, parametric_coordinates: [f64; P]) -> [f64; N]; 
    fn derivative_shape(&self, parametric_coordinates: [f64; P]) -> [f64; N]; 

    fn vertices_coordinates(&self) -> [[f64; D]; N];
    fn coordinates(&self, parametric_coordinates: [f64; P]) -> [f64; D];
    fn jacobe(&self, parametric_coordinates: [f64; P]) -> f64;
    fn jacobe_mat(&self, parametric_coordinates: [f64; P]) -> [[f64; P]; D];
}
