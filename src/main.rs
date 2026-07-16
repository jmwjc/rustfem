#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
use faer::prelude::*;
#[cfg(feature = "sparse")]
use faer::sparse::*;
use faer::{Side};

mod problem;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let np: usize = 11;
    let ne: usize = np-1;
    let l: f64 = 1.0;
    let x = Col::<f64>::from_fn(np, |i| i as f64 * l / (np-1) as f64);
    let mut f = Col::<f64>::zeros(np);

    #[cfg(feature = "std")]
    {
    let mut k = Mat::<f64>::zeros(np,np);
    for i in 0..ne {
        let x1 = x[i];
        let x2 = x[i+1];
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
        let x1 = x[i];
        let x2 = x[i+1];
        let dl = x2-x1;
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