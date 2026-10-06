//! `rustfem` — 一个用 Rust 编写的有限元方法（FEM）库。
//!
//! 提供模块化的单元（Element）、材料/问题操作（Operation）与网格 I/O 接口。
//!
//! 主要模块：
//!
//! - [`node`]：节点类型。
//! - [`element`]：单元 trait（形函数、雅可比等）及具体单元（`Seg2`、`Tri3`、`Poi1`）。
//! - [`operation`]：组装相关的抽象（`Variable`、`Elasticity`）。
//! - [`io`]：网格文件读取（GMSH）。

pub mod element;
pub mod io;
pub mod node;
pub mod operation;
