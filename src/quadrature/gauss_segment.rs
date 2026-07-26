use crate::quadrature::Quadrature;

pub struct GaussSeg1;

impl Quadrature<1,1> for GaussSeg1 {
    fn coordinates() -> [[f64; 1]; 1] {
        [
            [0.0],
        ]
    }
    fn weights() -> [f64; 1] {
        [
            2.0,
        ]
    }
}

pub struct GaussSeg2;

impl Quadrature<1,2> for GaussSeg2 {
    fn coordinates() -> [[f64; 1]; 2] {
        [
            [-0.5773502691896257],
            [ 0.5773502691896257],
        ]
    }
    fn weights() -> [f64; 2] {
        [
            1.0, 1.0,
        ]
    }
}