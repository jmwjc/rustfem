use crate::element::Element;
use crate::element::FullIntegration;
use crate::element::IntegrationScheme;
use crate::operation::Variable;
use crate::sparse_vector::Doublet;
use faer::sparse::Triplet;

/// 组装刚度矩阵，返回下三角（含对角线）的稀疏三元组。
///
/// `ea` 为轴向刚度（杨氏模量 × 横截面积）。
/// `v` 是一组 1D 单元（实现 [`Variable<1, P, N>`]），每个单元须实现
/// [`FullIntegration<P, G>`]。
pub fn stiffness<T, const P: usize, const N: usize, const G: usize>(
    v: &T,
    ea: f64,
) -> Vec<Triplet<usize, usize, f64>>
where
    T: Variable<1, P, N>,
    T::Item: Element<1, P, N> + FullIntegration<P, G>,
{
    let mut triplets: Vec<Triplet<usize, usize, f64>> = Vec::with_capacity(N * (N + 1) / 2 * G);
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

/// 施加自然边界条件（集中力），采用点单元（[`crate::element::poi1::Poi1`]）表示作用点。
///
/// `p` 为力值，`v` 为一组点单元（[`Variable<D, 0, 1>`]），每个点单元对应一个
/// 作用点，在其上施加 `Fᵢ += Nᵢ P`。返回 [`Doublet`] 列表，调用方可用
/// [`crate::sparse_vector::try_new_from_doublet`] 合并为载荷向量。
pub fn traction<T, const D: usize>(v: &T, p: f64) -> Vec<Doublet<usize, f64>>
where
    T: Variable<D, 0, 1>,
    T::Item: Element<D, 0, 1>,
{
    let mut doublets = Vec::new();
    for elm in v.iter() {
        let id = elm.id();
        let shape = elm.shape([]);
        for i in 0..1 {
            doublets.push(Doublet::new(id[i], shape[i] * p));
        }
    }
    doublets
}

/// 施加体力（分布载荷），采用算子形式（单元积分）。
///
/// `f` 为体力密度函数（每单位长度的轴向力），对每个单元积分
/// `Fᵢ = ∫ Nᵢ f dx = Σ_g Nᵢ(ξ_g) f(x_g) |J| w_g`。
/// 返回 [`Doublet`] 列表，调用方可用 [`crate::sparse_vector::try_new_from_doublet`]
/// 合并为载荷向量。
pub fn body_force<T, const P: usize, const N: usize, const G: usize>(
    v: &T,
    f: impl Fn([f64; 1]) -> f64,
) -> Vec<Doublet<usize, f64>>
where
    T: Variable<1, P, N>,
    T::Item: Element<1, P, N> + FullIntegration<P, G>,
{
    let ξ = T::Item::points();
    let w = T::Item::weights();
    let mut doublets = Vec::with_capacity(N * G);
    for elm in v.iter() {
        let id = elm.id();
        for g in 0..G {
            let ξg = ξ[g];
            let wg = w[g];
            let shape = elm.shape(ξg);
            let jacobe = elm.jacobe(ξg);
            let fg = f(elm.coordinates(ξg));
            for i in 0..N {
                doublets.push(Doublet::new(id[i], shape[i] * fg * jacobe * wg));
            }
        }
    }
    doublets
}

/// 用罚函数法施加位移约束（本质边界条件），采用点单元（[`crate::element::poi1::Poi1`]）表示约束点。
///
/// `penalty` 为罚数，`ū` 为指定位移。`v` 为一组点单元（[`Variable<D, 0, 1>`]），
/// 每个点单元对应一个约束点，在其上施加 `Kᵢⱼ += α Nᵢ Nⱼ`、`Fᵢ += α Nᵢ ū`。
/// 返回 `(刚度三元组, [`Doublet`] 载荷项)`。
pub fn displacement_penalty<T, const D: usize>(
    v: &T,
    penalty: f64,
    u: f64,
) -> (Vec<Triplet<usize, usize, f64>>, Vec<Doublet<usize, f64>>)
where
    T: Variable<D, 0, 1>,
    T::Item: Element<D, 0, 1>,
{
    let mut triplets = Vec::new();
    let mut doublets = Vec::new();
    for elm in v.iter() {
        let id = elm.id();
        let shape = elm.shape([]);
        for i in 0..1 {
            for j in 0..i + 1 {
                triplets.push(Triplet::new(id[i], id[j], penalty * shape[i] * shape[j]));
            }
            doublets.push(Doublet::new(id[i], penalty * shape[i] * u));
        }
    }
    (triplets, doublets)
}
