mod matrix;
use crate::matrix::Matrix;

fn main() {
    println!("Hello, world!");
    
    let mat_A = Matrix::from(
        (1..=8).collect(),
        4,
        2
    );

    println!("{}",mat_A.to_string())
}
