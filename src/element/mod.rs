pub mod poi1;
pub mod seg2;
pub mod tri3;

pub trait Element<const D: usize, const P: usize, const N: usize> {
    fn id(&self) -> [usize; N];
    fn shape(&self, parametric_coordinates: [f64; P]) -> [f64; N];
    fn derivative_shape(&self, parametric_coordinates: [f64; P]) -> [f64; N];

    fn vertices_coordinates(&self) -> [[f64; D]; N];
    fn coordinates(&self, parametric_coordinates: [f64; P]) -> [f64; D];
    fn jacobe(&self, parametric_coordinates: [f64; P]) -> f64;
    fn jacobe_mat(&self, parametric_coordinates: [f64; P]) -> [[f64; P]; D];
}

pub trait IntegrationScheme<const P: usize, const G: usize> {
    fn points() -> [[f64; P]; G];
    fn weights() -> [f64; G];
}

pub trait FullIntegration<const P: usize, const G: usize>: IntegrationScheme<P, G> {}

pub trait ReducedIntegration<const P: usize, const G: usize>: IntegrationScheme<P, G> {}

impl<T> IntegrationScheme<1, 2> for T {
    fn points() -> [[f64; 1]; 2] {
        [[-0.5773502691896257], [0.5773502691896257]]
    }
    fn weights() -> [f64; 2] {
        [1.0, 1.0]
    }
}

impl<T> IntegrationScheme<1, 1> for T {
    fn points() -> [[f64; 1]; 1] {
        [[0.0]]
    }
    fn weights() -> [f64; 1] {
        [2.0]
    }
}
