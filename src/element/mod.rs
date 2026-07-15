use crate::approximation::Approximation;

pub mod seg2;
pub use seg2::Seg2;

pub struct Seg2 {
    pub vertices: [usize; 2],
}