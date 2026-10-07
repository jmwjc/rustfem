use crate::element::{Element, FullIntegration, IntegrationScheme, Point, ReducedIntegration};
use crate::node::Node;

pub struct Poi1<const D: usize> {
    pub nodes: Node<D>,
}

impl<const D: usize> Poi1<D> {
    pub fn new(node: Node<D>) -> Self {
        Poi1 { nodes: node }
    }
}

impl<const D: usize> IntegrationScheme<0, 1> for Poi1<D> {
    fn points() -> [[f64; 0]; 1] {
        <Point as IntegrationScheme<0, 1>>::points()
    }
    fn weights() -> [f64; 1] {
        <Point as IntegrationScheme<0, 1>>::weights()
    }
}

// 点单元的完全积分与减缩积分相同：零维单点积分。
impl<const D: usize> FullIntegration<0, 1> for Poi1<D> {}
impl<const D: usize> ReducedIntegration<0, 1> for Poi1<D> {}

impl<const D: usize> Element<D, 0, 1> for Poi1<D> {
    fn id(&self) -> [usize; 1] {
        [self.nodes.id]
    }

    /// 点单元形函数恒为 1（零维参数空间无坐标）。
    fn shape(&self, _parametric_coordinates: [f64; 0]) -> [f64; 1] {
        [1.0]
    }

    /// 形函数为常数，导数为 0。
    fn derivative_shape(&self, _parametric_coordinates: [f64; 0]) -> [f64; 1] {
        [0.0]
    }

    fn vertices_coordinates(&self) -> [[f64; D]; 1] {
        [self.nodes.coordinates]
    }

    fn coordinates(&self, _parametric_coordinates: [f64; 0]) -> [f64; D] {
        self.nodes.coordinates
    }

    /// 零维点单元无拉伸，雅可比恒为 1。
    fn jacobe(&self, _parametric_coordinates: [f64; 0]) -> f64 {
        1.0
    }

    /// 零维参数空间到 D 维物理空间的雅可比矩阵为 `D × 0` 空矩阵。
    fn jacobe_mat(&self, _parametric_coordinates: [f64; 0]) -> [[f64; 0]; D] {
        std::array::from_fn(|_| [])
    }
}

#[cfg(test)]
mod tests {
    use super::Poi1;
    use crate::element::Element;
    use crate::node::Node;

    #[test]
    fn poi1_shape_and_coordinates() {
        let p = Poi1::new(Node::new(3, [1.0, 2.0]));
        assert_eq!(p.id(), [3]);
        assert_eq!(p.shape([]), [1.0]);
        assert_eq!(p.derivative_shape([]), [0.0]);
        assert_eq!(p.coordinates([]), [1.0, 2.0]);
        assert_eq!(p.jacobe([]), 1.0);
    }
}
