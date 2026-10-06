#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use faer::Side;
use faer::prelude::*;
use faer::sparse::{SparseColMat, Triplet};

use rustfem::element::seg2::Seg2;
use rustfem::element::tri3::Tri3;
use rustfem::io::gmsh;
use rustfem::node::Node;
use rustfem::operation::truss::Elasticity;
use std::time::Instant;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let start = Instant::now();

    // ---------- 1. GMSH 读取 ----------
    let mesh = gmsh::read_gmsh::<2>("test/msh/patchtest.msh");
    let segs: Vec<Seg2<2>> = mesh.elements("Γᵍ");
    let tris: Vec<Tri3<2>> = mesh.elements("Ω");
    println!("segments (Γᵍ): {}", segs.len());
    println!("triangles (Ω): {}", tris.len());

    // ---------- 2. 1D 桁架组装 + 求解 ----------
    // 构造一根由 10 个 Seg2<1> 单元组成的 1D 杆，左端固定、右端受单位拉力。
    let np = 11;
    let ne = np - 1;
    let bars: Vec<Seg2<1>> = (0..ne)
        .map(|i| {
            Seg2::new(
                Node::new(i, [i as f64 / ne as f64]),
                Node::new(i + 1, [(i + 1) as f64 / ne as f64]),
            )
        })
        .collect();

    let prop = Elasticity::new(1.0, 1.0, 1.0);
    let mut triplets = prop.stiffness(&bars);

    // 左端固定：罚函数法，在 (0,0) 上加一个大刚度。
    let alpha: f64 = 1e7;
    triplets.push(Triplet::new(0, 0, alpha));

    let k = SparseColMat::<usize, f64>::try_new_from_triplets(np, np, &triplets).unwrap();
    let mut f = Col::<f64>::zeros(np);
    f[np - 1] += 1.0;

    let llt = k.sp_cholesky(Side::Lower).unwrap();
    let d = llt.solve(&f);

    println!("truss displacement: {:?}", d);

    println!("Time elapsed: {:?}", start.elapsed());
}
