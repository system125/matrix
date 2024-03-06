use crate::matrix::Matrix;

mod matrix;
fn main() {
    println!("Hello, world!");
    
    let mat_A = Matrix::from(
        (1..8).collect(),
        2,
        4
    );

    println!("{:?}",mat_A.to_string())
}
