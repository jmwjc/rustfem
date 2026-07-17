// use std::{array, mem::{self, MaybeUninit}};

use faer::sparse::*;
use crate::approximation::Approximation;

// pub trait BilinearForm<V: Variable, const N: usize> {
//     fn assemble(&self, v: &V, u: &V) -> &vec[Tri];
//     fn evaluate(&self, v: &V, u: &V) -> f64;
// }

// pub trait LinearForm<V: Variable, const N: usize> {
//     fn assemble(&self, v: &V) -> [f64; N];
//     fn evaluate(&self, v: &V) -> f64;
// }

pub fn bilinear_gradient_gradient_1<const NDOF: usize>(
    c: f64,
    elm: &impl Approximation<1, NDOF>,
) -> Vec<Triplet<usize, usize, f64>> 
{
    let id = elm.id();
    let n = id.len();
    let B = elm.shape()

    let mut triplets = Vec::with_capacity(n * (n + 1) / 2);
    
    for i in 0..n {
        for j in 0..=i {
            triplets.push(Triplet::new(id[i], id[j], 0.0));
        }
    }
    triplets
}
