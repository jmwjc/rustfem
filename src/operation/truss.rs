use faer::sparse::Triplet;
use crate::element::Element;
use crate::operation::Variable;
use crate::operation::LinearForm;
use crate::operation::BilinearForm;
use crate::quadrature::Quadrature;

pub struct Truss {
    young_modulus: f64,
    cross_sectional_area: f64,
    moment_of_inertia: f64,
}

impl Truss {
    pub fn new(e:f64, a:f64, i: f64) -> Self {
        Truss { young_modulus: e, cross_sectional_area: a, moment_of_inertia: i }
    }
}

impl BilinearForm<1> for Truss {
    fn assemble<T, S, const P: usize, const N: usize, const G: usize>(&self, v: &T, quadrature: S) -> Vec<Triplet<usize, usize, f64>> where
        T: Variable<P, N>,
        T::Item: Element<P, N>,
        S: Quadrature<P, G>,
    {
        let mut triplets: Vec<Triplet<usize, usize, f64>> = Vec::with_capacity(N*(N+1)/2*G);
        let ea = self.young_modulus*self.cross_sectional_area;
        let ξ = quadrature.coordinates();
        let w = quadrature.weights();
        for elm in v.iter() {
            let id = elm.id();
            for g in 0..G {
                let ξg = ξ[g];
                let wg = w[g];
                let dshape = elm.derivative_shape(ξg);
                let jacobe = elm.jacobe(ξg);
                for i in 0..N {
                    for j in 0..i+1 {
                        triplets.push(Triplet::new(id[i], id[j], ea*dshape[i]*dshape[j]*jacobe*wg));
                    }
                }
            }
        }
        triplets
    }
}