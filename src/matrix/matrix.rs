
use std::fmt;
use core::option::Iter;

use super::matrix_row_iter::matrix_row_iter;
/**
 * Matrix implemented by using linear array
 */
pub struct Matrix {
    pub values: Vec<i32>,
    pub height: usize,
    pub width: usize
}

impl Matrix {
    pub fn from(data:Vec<i32>,width:usize,height:usize)->Matrix{
        Matrix {
            values: data,
            height: height,
            width
        }
    }

    pub fn to_iter(&self)-> impl Iterator<Item = &i32> {
         self.values.iter()
    }
    pub fn to_mut_iter(&mut self) -> impl Iterator<Item = &mut i32>{
        self.values.iter_mut()
    }

    pub fn to_row_iter<'a> (&'a self) -> matrix_row_iter<'a> {
        matrix_row_iter::new(&self)
    }
}
 