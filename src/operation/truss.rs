use crate::element::Element;
use crate::element::FullIntegration;
use crate::element::IntegrationScheme;
use crate::operation::Variable;
use faer::sparse::Triplet;

/// 线弹性材料属性（1D 桁架/杆）。
pub struct Elasticity {
    young_modulus: f64,
    cross_sectional_area: f64,
    density: f64,
}

#[allow(non_snake_case)]
impl Elasticity {
    pub fn new(E: f64, A: f64, rho: f64) -> Self {
        Elasticity {
            young_modulus: E,
            cross_sectional_area: A,
            density: rho,
        }
    }

    /// 组装刚度矩阵，返回下三角（含对角线）的稀疏三元组。
    ///
    /// `v` 是一组 1D 单元（实现 [`Variable<1, P, N>`]），每个单元须实现
    /// [`FullIntegration<P, G>`]。
    pub fn stiffness<T, const P: usize, const N: usize, const G: usize>(
        &self,
        v: &T,
    ) -> Vec<Triplet<usize, usize, f64>>
    where
        T: Variable<1, P, N>,
        T::Item: Element<1, P, N> + FullIntegration<P, G>,
    {
        let mut triplets: Vec<Triplet<usize, usize, f64>> =
            Vec::with_capacity(N * (N + 1) / 2 * G);
        let ea = self.young_modulus * self.cross_sectional_area;
        let ξ = T::Item::points();
        let w = T::Item::weights();
        for elm in v.iter() {
            let id = elm.id();
            for g in 0..G {
                let ξg = ξ[g];
                let wg = w[g];
                let dshape = elm.derivative_shape(ξg);
                let jacobe = elm.jacobe(ξg);
                for i in 0..N {
                    for j in 0..i + 1 {
                        triplets.push(Triplet::new(
                            id[i],
                            id[j],
                            ea * dshape[i] * dshape[j] * jacobe * wg,
                        ));
                    }
                }
            }
        }
        triplets
    }
}
