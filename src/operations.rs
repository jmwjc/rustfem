use crate::variable::Variable;
use faer::sparse::*;

pub trait BilinearForm<V: Variable, const N: usize> {
    fn assemble(&self, v: &V, u: &V) -> &vec[Tri];
    fn evaluate(&self, v: &V, u: &V) -> f64;
}

pub trait LinearForm<V: Variable, const N: usize> {
    fn assemble(&self, v: &V) -> [f64; N];
    fn evaluate(&self, v: &V) -> f64;
}