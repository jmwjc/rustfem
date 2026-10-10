//! `rustfem` — a finite element method (FEM) library written in Rust.
//!
//! Provides modular interfaces for elements, material/problem operations, and mesh I/O.
//!
//! Main modules:
//!
//! - [`node`]: node types.
//! - [`element`]: the element trait (shape functions, Jacobian, etc.) and concrete elements (`Seg2`, `Tri3`, `Poi1`).
//! - [`operation`]: assembly-related abstractions (`Variable` and the stiffness, load, and penalty assembly operators).
//! - [`io`]: mesh file reading (GMSH).

pub mod element;
pub mod io;
pub mod node;
pub mod operation;
pub mod sparse_vector;
