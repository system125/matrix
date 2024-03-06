
use super::Matrix;
use std::option::Option;

pub struct matrix_row_iter<'a> {
    matrix: &'a Matrix,
    current_row: usize
}

impl<'a> matrix_row_iter <'a>{
    /**
     * Takes in matrix and returns iterator which iterates with each row
     */
    pub fn new(matrix:&'a Matrix) -> matrix_row_iter<'a> {
        matrix_row_iter{
            matrix: matrix,
            current_row: 0
        }
    }
}

impl<'a> Iterator for matrix_row_iter<'a>{
    type Item = &'a [i32];

    fn next (&mut self) -> Option<Self::Item>{
        let mat = &self.matrix.values;
        let width = self.matrix.width;
        let curr_row = self.current_row;
        let height = self.matrix.height;

        if curr_row == self.matrix.height {
            return None
        }
        
        self.current_row += 1;
        let start_indx = curr_row * width;
        let end_indx = (curr_row + 1) * (width);
        Some(&self.matrix.values[start_indx..end_indx])
    }
}