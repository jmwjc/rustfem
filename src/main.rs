#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use faer::Side;
use faer::prelude::*;
use faer::sparse::SparseColMat;

use rustfem::element::poi1::Poi1;
use rustfem::element::seg2::Seg2;
use rustfem::element::tri3::Tri3;
use rustfem::io::gmsh;
use rustfem::node::Node;
use rustfem::operation::bar::{body_force, displacement_penalty, stiffness, traction};
use rustfem::sparse_vector::try_new_from_doublet;
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
    // 构造一根由 10 个 Seg2<1> 单元组成的 1D 杆，左端固定，
    // 受自重（均布体力 f = 1）与右端集中拉力 P = 1。
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

    // 左端约束点、右端作用点（点单元）。
    let left = Poi1::new(Node::new(0, [0.0]));
    let right = Poi1::new(Node::new(np - 1, [1.0]));

    let mut triplets = stiffness(&bars, 1.0);

    // 左端固定：罚函数法施加位移约束 u(0) = 0。
    let alpha: f64 = 1e7;
    let (penalty_triplets, penalty_doublets) = displacement_penalty(&vec![left], alpha, 0.0);
    triplets.extend(penalty_triplets);

    let k = SparseColMat::<usize, f64>::try_new_from_triplets(np, np, &triplets).unwrap();

    // 均布体力（自重）：自然边界条件（体力）。
    let mut doublets = body_force(&bars, |_| 1.0);
    // 右端受单位拉力：自然边界条件（集中力）。
    doublets.extend(traction(&vec![right], 1.0));
    // 罚函数载荷。
    doublets.extend(penalty_doublets);
    let f = try_new_from_doublet(np, &doublets);

    let llt = k.sp_cholesky(Side::Lower).unwrap();
    let d = llt.solve(&f);

    println!("truss displacement: {:?}", d);

    println!("Time elapsed: {:?}", start.elapsed());
}
