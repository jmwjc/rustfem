#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

mod node;
mod element;
mod operation;
mod quadrature;

use std::time::Instant;
use faer::prelude::*;
use faer::sparse::*;
use faer::{Side};

use crate::node::Node;
use crate::element::seg2::Seg2;
use crate::operation::truss::Truss;
use crate::operation::BilinearForm;
use crate::quadrature::gauss_segment::GaussSeg1;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let start = Instant::now();
    let np = 11;
    let ne = np-1;
    let u:Vec<Seg2<1>> = (0..ne).map(|i|Seg2::new(Node::new(i, [i as f64 * 1.0/ne as f64]), Node::new(i+1, [(i+1) as f64 * 1.0/ne as f64]))).collect();
    let a = Truss::new(1.0, 1.0, 1.0);
    let mut triplets = a.assemble(u, GaussSeg1);

    let alpha: f64 = 1e7;
    triplets.push(Triplet::new(0, 0, alpha));

    let k = SparseColMat::<usize, f64>::try_new_from_triplets(np, np, &triplets).unwrap();
    let mut f = Col::<f64>::zeros(np);
    f[np-1] += 1.0;

    let llt = k.sp_cholesky(Side::Lower).unwrap();
    let d = llt.solve(&f);

    // let lu = k.sp_lu().unwrap();
    // let d = lu.solve(&f);
    println!("{:?}", k);
    println!("{:?}", f);
    println!("{:?}", d);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    // }
}