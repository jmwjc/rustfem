#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
use faer::prelude::*;
#[cfg(feature = "sparse")]
use faer::sparse::*;
use faer::{Side};


struct Node<const D: usize> {
    id: usize,
    coordinates: [f64; D],
}

struct Seg2<const D: usize> {
    nodes: [Node<D>; 2]
}

impl<const D:usize> Approximation<D, 2> for Seg2<D> {
    fn id(&self): [usize; 2] { std::array::from_fn(|i| self.nodes[i].id)}
    fn shape(&self, ξ: [f64; D]) -> [f64; 2] {
        [0.5*(1.0-ξ), 0.5*(1.0+ξ)]
    }
}

struct Poi1<const D: usize> {
    nodes: Node<D>
}

struct TrussStiffness {
    young_modulus: f64,
    cross_sectional_area: f64,
}

trait Approximation<const D: usize, const P: usize> {
    fn id(&self) -> [usize; P];
    fn shape(&self, ξ: [f64; D]) -> [f64; P]; 
}

trait Variable: IntoIterator {

}

impl Variable for Vec<Seg2<1>> {}

trait BilinearForm {
    fn assemble<T>(v: T) -> Vec<Triplet<usize, usize, f64>>
    where 
        T: Variable,
        // T::Item: Element,
    ;
}

impl BilinearForm for TrussStiffness {
    fn assemble<T: Variable>(v: T) -> Vec<Triplet<usize, usize, f64>> {
        let mut triplets: Vec<Triplet<usize, usize, f64>> = Vec::new();
        for elm in v {
            let id = elm.id()
        }
        triplets
    }
}

trait LinearForm {
    fn assemble<T: Variable>(v: T) -> Vec<(usize, f64)>;
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let np: usize = 11;
    let ne: usize = np-1;
    let l: f64 = 1.0;
    let elements: Vec<element::Seg2> = (0..ne).map(|i| element::Seg2::new(approximation::Node { id: i, x: l/ne as f64 * i as f64, y: 0.0, z: 0.0 }, approximation::Node { id: i+1, x: l/ne as f64 * (i+1) as f64, y: 0.0, z: 0.0 })).collect();
    // let x = Col::<f64>::from_fn(np, |i| i as f64 * l / (np-1) as f64);
    let mut f = Col::<f64>::zeros(np);

    #[cfg(feature = "std")]
    {
    let mut k = Mat::<f64>::zeros(np,np);
    for i in 0..ne {
        let x1 = nodes[i].x;
        let x2 = nodes[i+1].x;
        let dl = x2-x1;
        k[(i,i)] += 1.0/dl;
        k[(i,i+1)] -= 1.0/dl;
        k[(i+1,i)] -= 1.0/dl;
        k[(i+1,i+1)] += 1.0/dl;
    }
    let alpha: f64 = 1e7;
    k[(1,1)] += alpha;
    f[np-1] += 1.0;

    let llt = k.llt(Side::Lower).unwrap();

    let d = llt.solve(&f);
    println!("{:?}", d)
    }
    #[cfg(feature = "sparse")]
    {
    let mut triplets: Vec<Triplet<usize, usize, f64>> = Vec::new();
    for i in 0..ne {
        let x1 = elements[i].dofs[0].x;
        let x2 = elements[i].dofs[1].x;
        let dl = x2-x1;
        // let id = 
        triplets.push(Triplet::new(i, i, 1.0 / dl));
        // triplets.push(Triplet::new(i, i+1, -1.0 / dl));
        triplets.push(Triplet::new(i+1, i, -1.0 / dl));
        triplets.push(Triplet::new(i+1, i+1, 1.0 / dl));
    }
    let alpha: f64 = 1e7;
    triplets.push(Triplet::new(0, 0, alpha));

    let k = SparseColMat::<usize, f64>::try_new_from_triplets(np, np, &triplets).unwrap();
    f[np-1] += 1.0;

    let llt = k.sp_cholesky(Side::Lower).unwrap();
    let d = llt.solve(&f);

    // let lu = k.sp_lu().unwrap();
    // let d = lu.solve(&f);
    println!("{:?}", d)
    }
}