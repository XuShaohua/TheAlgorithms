// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt;
use std::fmt::Formatter;

use crate::traits::IsZero;

pub struct MatrixElement<T: IsZero> {
    pub row: usize,
    pub column: usize,
    pub value: T,
}

pub struct ArraySparseMatrix<T: IsZero> {
    pub vec: Vec<MatrixElement<T>>,
}

impl<T: IsZero> ArraySparseMatrix<T> {
    #[must_use]
    pub fn construct<I, I2>(sparse_matrix: I) -> Self
    where
        I: IntoIterator<Item=I2>,
        I2: IntoIterator<Item=T>,
    {
        let mut vec = Vec::new();

        for (row, row_list) in sparse_matrix.into_iter().enumerate() {
            for (column, element) in row_list.into_iter().enumerate() {
                if element.is_not_zero() {
                    let element = MatrixElement {
                        row,
                        column,
                        value: element,
                    };
                    vec.push(element);
                }
            }
        }
        Self { vec }
    }
}

impl<T: fmt::Debug + IsZero> fmt::Debug for MatrixElement<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MatrixElement")
            .field("row", &self.row)
            .field("column", &self.column)
            .field("value", &self.value)
            .finish()
    }
}

impl<T: fmt::Debug + IsZero> fmt::Debug for ArraySparseMatrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(&self.vec).finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::array_sparse_matrix::ArraySparseMatrix;

    #[test]
    fn test_array() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0]
        ];
        let sm = ArraySparseMatrix::construct(MATRIX);
        println!("sm: {sm:?}");
        assert_eq!(sm.vec.len(), 6);
    }

    #[test]
    fn test_vec_of_array() {
        let matrix = vec![
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0],
        ];
        let sm = ArraySparseMatrix::construct(matrix);
        println!("sm: {sm:?}");
        assert_eq!(sm.vec.len(), 6);
    }

    #[test]
    fn test_vec_of_vec() {
        let matrix = vec![
            vec![0, 0, 3, 0, 4],
            vec![0, 0, 5, 7, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 2, 6, 0, 0],
        ];
        let sm = ArraySparseMatrix::construct(matrix);
        println!("sm: {sm:?}");
        assert_eq!(sm.vec.len(), 6);
    }
}