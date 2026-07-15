use crate::variable::Variable;

pub trait BilinearForm<V: Variable, const N: usize> {
    fn assemble(&self, v: &V, u: &V) -> [f64; N];
    fn evaluate(&self, v: &V, u: &V) -> f64;
}

pub trait LinearForm<V: Variable, const N: usize> {
    fn assemble(&self, v: &V) -> [f64; N];
    fn evaluate(&self, v: &V) -> f64;
}