pub mod truss;

use faer::sparse::Triplet;
use crate::element::Element;
use crate::quadrature::Quadrature;

pub trait Variable <const D: usize, const P: usize, const N: usize> {
    type Item: Element<D, P, N>;
    fn iter(&self) -> impl Iterator<Item = &Self::Item>;
}
impl<T, const D: usize, const P: usize, const N: usize> Variable<D, P, N> for &[T]
where
    T: Element<D, P, N>,
{
    type Item = T;
    
    fn iter(&self) -> impl Iterator<Item = &T> {
        <[T]>::iter(self)
    }
}
impl<T, const D: usize, const P: usize, const N: usize> Variable<D, P, N> for Vec<T> where T: Element<D, P, N> {
    type Item = T;
    fn iter(&self) -> impl Iterator<Item = &T> {
        self.as_slice().iter()
    }
}

trait LinearForm<const D: usize> {
    fn assemble<T, S, const P: usize, const N: usize, const G: usize>(&self, v: T, quadrature: S) -> Vec<(usize, f64)>
    where
        T: Variable<D, P, N>,
        T::Item: Element<D, P, N>,
        S: Quadrature<P, G>,
    ;
}

pub trait BilinearForm<const D: usize> {
    fn assemble<T, S, const P: usize, const N: usize, const G: usize>(&self, v: &T, quadrature: S) -> Vec<Triplet<usize, usize, f64>>
    where 
        T: Variable<D, P, N>,
        T::Item: Element<D, P, N>,
        S: Quadrature<P, G>,
    ;
}