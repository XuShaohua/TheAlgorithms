// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;

use crate::traits::IsZero;

/// 数组中的每个元素节点
pub struct MatrixElement<T: IsZero> {
    // 在矩阵中的行号
    pub row: usize,
    // 在矩阵中的列号
    pub column: usize,
    // 元素的值
    pub value: T,
}

/// 使用数组来存储稀疏矩阵
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

    #[must_use]
    pub fn value(&self, row: usize, column: usize) -> Option<T> {
        let result = self.find_element(row, column);
        result.ok().map(|index| self.vec[index].value)
    }

    fn find_element(&self, row: usize, column: usize) -> Result<usize, usize> {
        self.vec.binary_search_by(|node| {
            match node.row.cmp(&row) {
                Ordering::Equal => node.column.cmp(&column),
                order => order
            }
        })
    }

    #[must_use]
    pub fn value_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        let result = self.find_element(row, column);
        result.ok().map(|index| &mut self.vec[index].value)
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

    #[test]
    fn test_value() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0]
        ];
        let sm = ArraySparseMatrix::construct(MATRIX);
        assert_eq!(sm.value(0, 2), Some(3));
        assert_eq!(sm.value(1, 3), Some(7));
        assert_eq!(sm.value(1, 4), None);
    }

    #[test]
    fn test_value_mut() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0]
        ];
        let mut sm = ArraySparseMatrix::construct(MATRIX);
        let ret = sm.value_mut(0, 2);
        assert!(ret.is_some());
        if let Some(value) = ret {
            *value *= 2;
        }
        assert_eq!(sm.vec[0].value, 6);
    }
}