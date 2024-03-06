
use super::Matrix;
use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"Matrix to string")?;

        for i in self.to_row_iter() {
            for val in i.iter() {
                write!(f,"{:?} ",*val)?;
            }
            writeln!(f,"")?;
        }

        Ok(())
    }

}