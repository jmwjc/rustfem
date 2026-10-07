use crate::element::{
    Element, FullIntegration, IntegrationScheme, ReducedIntegration, TensorProductQuad,
};
use crate::node::Node;

pub struct Quad4<const D: usize> {
    nodes: [Node<D>; 4],
}

impl<const D: usize> Quad4<D> {
    pub fn new(n1: Node<D>, n2: Node<D>, n3: Node<D>, n4: Node<D>) -> Self {
        Quad4 {
            nodes: [n1, n2, n3, n4],
        }
    }
}

// 四边形完全积分：2×2 张量积（4 点，3 阶）；减缩积分：1 点（形心）。
impl<const D: usize> IntegrationScheme<2, 1> for Quad4<D> {
    fn points() -> [[f64; 2]; 1] {
        <TensorProductQuad<1> as IntegrationScheme<2, 1>>::points()
    }
    fn weights() -> [f64; 1] {
        <TensorProductQuad<1> as IntegrationScheme<2, 1>>::weights()
    }
}

impl<const D: usize> IntegrationScheme<2, 4> for Quad4<D> {
    fn points() -> [[f64; 2]; 4] {
        <TensorProductQuad<2> as IntegrationScheme<2, 4>>::points()
    }
    fn weights() -> [f64; 4] {
        <TensorProductQuad<2> as IntegrationScheme<2, 4>>::weights()
    }
}

impl<const D: usize> FullIntegration<2, 4> for Quad4<D> {}
impl<const D: usize> ReducedIntegration<2, 1> for Quad4<D> {}

impl<const D: usize> Element<D, 2, 4> for Quad4<D> {
    fn id(&self) -> [usize; 4] {
        [
            self.nodes[0].id,
            self.nodes[1].id,
            self.nodes[2].id,
            self.nodes[3].id,
        ]
    }

    /// 双线性形函数：节点 1、2、3、4 分别对应参考四边形顶点
    /// `(-1, -1)`、`(1, -1)`、`(1, 1)`、`(-1, 1)`。
    fn shape(&self, parametric_coordinates: [f64; 2]) -> [f64; 4] {
        let ξ = parametric_coordinates[0];
        let η = parametric_coordinates[1];
        [
            0.25 * (1.0 - ξ) * (1.0 - η),
            0.25 * (1.0 + ξ) * (1.0 - η),
            0.25 * (1.0 + ξ) * (1.0 + η),
            0.25 * (1.0 - ξ) * (1.0 + η),
        ]
    }

    /// 对第一个参数坐标 `ξ` 的偏导 `∂N/∂ξ`。
    ///
    /// 注：`Element` 的该方法返回 `[f64; N]`，无法容纳二维单元的完整梯度；
    /// 对 `η` 的偏导及物理梯度需结合 [`Self::jacobe_mat`] 另行求得。
    fn derivative_shape(&self, parametric_coordinates: [f64; 2]) -> [f64; 4] {
        let η = parametric_coordinates[1];
        [
            -0.25 * (1.0 - η),
            0.25 * (1.0 - η),
            0.25 * (1.0 + η),
            -0.25 * (1.0 + η),
        ]
    }

    fn vertices_coordinates(&self) -> [[f64; D]; 4] {
        [
            self.nodes[0].coordinates,
            self.nodes[1].coordinates,
            self.nodes[2].coordinates,
            self.nodes[3].coordinates,
        ]
    }

    fn coordinates(&self, parametric_coordinates: [f64; 2]) -> [f64; D] {
        let shape = self.shape(parametric_coordinates);
        let vertices = self.vertices_coordinates();
        std::array::from_fn(|i| (0..4).map(|k| vertices[k][i] * shape[k]).sum())
    }

    /// 参考四边形到物理四边形的面积比：`JᵀJ` 的行列式的平方根
    /// （`D = 2` 时即 `|det J|`，`D = 3` 时为两切向量的叉积模）。
    fn jacobe(&self, parametric_coordinates: [f64; 2]) -> f64 {
        let j = self.jacobe_mat(parametric_coordinates);
        let aa: f64 = (0..D).map(|i| j[i][0] * j[i][0]).sum();
        let bb: f64 = (0..D).map(|i| j[i][1] * j[i][1]).sum();
        let ab: f64 = (0..D).map(|i| j[i][0] * j[i][1]).sum();
        (aa * bb - ab * ab).sqrt()
    }

    /// `J[i][0] = ∂x_i/∂ξ`、`J[i][1] = ∂x_i/∂η`（双线性映射，随 `(ξ, η)` 变化）。
    fn jacobe_mat(&self, parametric_coordinates: [f64; 2]) -> [[f64; 2]; D] {
        let ξ = parametric_coordinates[0];
        let η = parametric_coordinates[1];
        let dn_dξ = [
            -0.25 * (1.0 - η),
            0.25 * (1.0 - η),
            0.25 * (1.0 + η),
            -0.25 * (1.0 + η),
        ];
        let dn_dη = [
            -0.25 * (1.0 - ξ),
            -0.25 * (1.0 + ξ),
            0.25 * (1.0 + ξ),
            0.25 * (1.0 - ξ),
        ];
        std::array::from_fn(|i| {
            [
                (0..4).map(|k| self.nodes[k].coordinates[i] * dn_dξ[k]).sum(),
                (0..4).map(|k| self.nodes[k].coordinates[i] * dn_dη[k]).sum(),
            ]
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Quad4;
    use crate::element::Element;
    use crate::node::Node;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-12
    }

    #[test]
    fn quad4_shape_and_jacobe() {
        // 单位正方形 (0,0)、(1,0)、(1,1)、(0,1)：面积 1，jacobe(0,0) = 1/4。
        let q = Quad4::new(
            Node::new(0, [0.0, 0.0]),
            Node::new(1, [1.0, 0.0]),
            Node::new(2, [1.0, 1.0]),
            Node::new(3, [0.0, 1.0]),
        );
        assert_eq!(q.id(), [0, 1, 2, 3]);
        // 顶点处的形函数取值
        assert_eq!(q.shape([-1.0, -1.0]), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(q.shape([1.0, -1.0]), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(q.shape([1.0, 1.0]), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(q.shape([-1.0, 1.0]), [0.0, 0.0, 0.0, 1.0]);
        // 形心 (ξ, η) = (0, 0) 映射到物理形心 (0.5, 0.5)
        let c = q.coordinates([0.0, 0.0]);
        assert!(close(c[0], 0.5));
        assert!(close(c[1], 0.5));
        // jacobe = |det J| = 1/4
        assert!(close(q.jacobe([0.0, 0.0]), 0.25));
    }

    #[test]
    fn quad4_jacobe_3d() {
        // 空间四边形 (0,0,0)、(2,0,0)、(2,3,0)、(0,3,0)：面积 6，jacobe = 6/4 = 1.5。
        let q = Quad4::new(
            Node::new(0, [0.0, 0.0, 0.0]),
            Node::new(1, [2.0, 0.0, 0.0]),
            Node::new(2, [2.0, 3.0, 0.0]),
            Node::new(3, [0.0, 3.0, 0.0]),
        );
        assert!(close(q.jacobe([0.0, 0.0]), 1.5));
    }
}
