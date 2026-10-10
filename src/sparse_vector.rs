//! Sparse vector assembly: `Doublet` entries and a constructor, analogous to the `Triplet` of faer's sparse matrices.

use faer::prelude::*;

/// An assembly entry for a sparse vector: a `(index, value)` pair, analogous to the `Triplet` of faer's sparse matrices.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Doublet<I, T> {
    pub index: I,
    pub value: T,
}

impl<I, T> Doublet<I, T> {
    pub fn new(index: I, value: T) -> Self {
        Doublet { index, value }
    }
}

/// 从 `Doublet` 列表构造稠密向量，重复索引的值累加。
///
/// `n` 为向量长度（自由度总数）。累加天然无冲突，故直接返回稠密向量
/// （类比 faer 的 [`faer::sparse::SparseColMat::try_new_from_triplets`]）。
pub fn try_new_from_doublet(n: usize, doublets: &[Doublet<usize, f64>]) -> Col<f64> {
    let mut v = Col::<f64>::zeros(n);
    for d in doublets {
        v[d.index] += d.value;
    }
    v
}
