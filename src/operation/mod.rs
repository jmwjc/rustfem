pub mod truss;

use faer::sparse::Triplet;
use crate::element::Element;
use crate::quadrature::Quadrature;

pub trait Variable <const P: usize, const N: usize>: IntoIterator where Self::Item: Element<P, N> {}
impl<T, const P: usize, const N: usize> Variable<P, N> for Vec<T> where T: Element<P, N> {}

trait LinearForm {
    fn assemble<T, S, const P: usize, const N: usize, const G: usize>(&self, v: T, quadrature: S) -> Vec<(usize, f64)>
    where
        T: Variable<P, N>,
        T::Item: Element<P, N>,
        S: Quadrature<P, G>,
    ;
}

pub trait BilinearForm<const D: usize> {
    fn assemble<T, S, const P: usize, const N: usize, const G: usize>(&self, v: T, quadrature: S) -> Vec<Triplet<usize, usize, f64>>
    where 
        T: Variable<P, N>,
        T::Item: Element<P, N>,
        S: Quadrature<P, G>,
    ;
}