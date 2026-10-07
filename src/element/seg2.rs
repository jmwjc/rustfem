use crate::element::{Element, FullIntegration, GaussLegendre1D, IntegrationScheme, ReducedIntegration};
use crate::node::Node;

pub struct Seg2<const D: usize> {
    nodes: [Node<D>; 2],
}

impl<const D: usize> Seg2<D> {
    pub fn new(n1: Node<D>, n2: Node<D>) -> Self {
        Seg2 { nodes: [n1, n2] }
    }
}

impl<const D: usize> IntegrationScheme<1, 1> for Seg2<D> {
    fn points() -> [[f64; 1]; 1] {
        <GaussLegendre1D<1> as IntegrationScheme<1, 1>>::points()
    }
    fn weights() -> [f64; 1] {
        <GaussLegendre1D<1> as IntegrationScheme<1, 1>>::weights()
    }
}

impl<const D: usize> IntegrationScheme<1, 2> for Seg2<D> {
    fn points() -> [[f64; 1]; 2] {
        <GaussLegendre1D<2> as IntegrationScheme<1, 2>>::points()
    }
    fn weights() -> [f64; 2] {
        <GaussLegendre1D<2> as IntegrationScheme<1, 2>>::weights()
    }
}

impl<const D: usize> ReducedIntegration<1, 1> for Seg2<D> {}
impl<const D: usize> FullIntegration<1, 2> for Seg2<D> {}

impl<const D: usize> Element<D, 1, 2> for Seg2<D> {
    fn id(&self) -> [usize; 2] {
        [self.nodes[0].id, self.nodes[1].id]
    }
    fn shape(&self, parametric_coordinates: [f64; 1]) -> [f64; 2] {
        let ξ = parametric_coordinates[0];
        [0.5 * (1.0 - ξ), 0.5 * (1.0 + ξ)]
    }
    fn derivative_shape(&self, parametric_coordinates: [f64; 1]) -> [f64; 2] {
        let l = 2.0 * self.jacobe(parametric_coordinates);
        [-1.0 / l, 1.0 / l]
    }
    fn vertices_coordinates(&self) -> [[f64; D]; 2] {
        [self.nodes[0].coordinates, self.nodes[1].coordinates]
    }
    fn coordinates(&self, parametric_coordinates: [f64; 1]) -> [f64; D] {
        let vertics_coordinates = self.vertices_coordinates();
        let shape = self.shape(parametric_coordinates);
        std::array::from_fn(|i| {
            vertics_coordinates[0][i] * shape[0] + vertics_coordinates[1][i] * shape[1]
        })
    }
    fn jacobe(&self, _parametric_coordinates: [f64; 1]) -> f64 {
        (0..D)
            .map(|i| (self.nodes[0].coordinates[i] - self.nodes[1].coordinates[i]).powi(2))
            .sum::<f64>()
            .sqrt()
            * 0.5
    }
    fn jacobe_mat(&self, _parametric_coordinates: [f64; 1]) -> [[f64; 1]; D] {
        std::array::from_fn(|i| {
            [0.5 * (self.nodes[1].coordinates[i] - self.nodes[0].coordinates[i])]
        })
    }
}
