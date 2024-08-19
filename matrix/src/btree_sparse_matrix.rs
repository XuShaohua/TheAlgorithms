// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::BTreeMap;

use crate::traits::IsZero;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MatrixIndex {
    row: usize,
    column: usize,
}

/// Store sparse matrix with btree.
#[derive(Debug, Default, Clone)]
pub struct BTreeSparseMatrix<T: IsZero> {
    map: BTreeMap<MatrixIndex, T>,
}

impl<T: IsZero> BTreeSparseMatrix<T> {
    #[must_use]
    pub fn construct<I, I2>(sparse_matrix: I) -> Self
    where
        I: IntoIterator<Item=I2>,
        I2: IntoIterator<Item=T>,
    {
        let mut map = BTreeMap::new();

        for (row, row_list) in sparse_matrix.into_iter().enumerate() {
            for (column, element) in row_list.into_iter().enumerate() {
                if element.is_not_zero() {
                    map.insert(MatrixIndex { row, column }, element);
                }
            }
        }
        Self { map }
    }

    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Get node value at (row, column).
    #[must_use]
    pub fn value(&self, row: usize, column: usize) -> Option<T> {
        self.map.get(&MatrixIndex { row, column }).copied()
    }

    /// Get mutable reference to node value at (row, column).
    #[must_use]
    pub fn value_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.map.get_mut(&MatrixIndex { row, column })
    }

    /// If found old node at (row, column), returns old value; otherwise returns None.
    pub fn add_element(&mut self, row: usize, column: usize, value: T) -> Option<T> {
        self.map.insert(MatrixIndex { row, column }, value)
    }

    /// If found node at (row, column), returns value of that node; otherwise returns None.
    pub fn remove_element(&mut self, row: usize, column: usize) -> Option<T> {
        self.map.remove(&MatrixIndex { row, column })
    }
}

#[cfg(test)]
mod tests {
    use super::BTreeSparseMatrix;

    #[test]
    fn test_array() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0]
        ];
        let sm = BTreeSparseMatrix::construct(MATRIX);
        println!("sm: {sm:?}");
        assert_eq!(sm.len(), 6);
    }

    #[test]
    fn test_vec_of_array() {
        let matrix = vec![
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0],
        ];
        let sm = BTreeSparseMatrix::construct(matrix);
        println!("sm: {sm:?}");
        assert_eq!(sm.len(), 6);
    }

    #[test]
    fn test_vec_of_vec() {
        let matrix = vec![
            vec![0, 0, 3, 0, 4],
            vec![0, 0, 5, 7, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 2, 6, 0, 0],
        ];
        let sm = BTreeSparseMatrix::construct(matrix);
        println!("sm: {sm:?}");
        assert_eq!(sm.len(), 6);
    }

    #[test]
    fn test_value() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0]
        ];
        let sm = BTreeSparseMatrix::construct(MATRIX);
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
        let mut sm = BTreeSparseMatrix::construct(MATRIX);
        let ret = sm.value_mut(0, 2);
        assert!(ret.is_some());
        if let Some(value) = ret {
            *value *= 2;
        }
        assert_eq!(sm.value(0, 2), Some(6));
    }

    #[test]
    fn test_add_element() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0]
        ];
        let mut sm = BTreeSparseMatrix::construct(MATRIX);
        sm.add_element(1, 0, 1);
        assert_eq!(sm.len(), 7);
        assert_eq!(sm.value(1, 0), Some(1));
    }

    #[test]
    fn test_remove_element() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0]
        ];
        let mut sm = BTreeSparseMatrix::construct(MATRIX);
        let ret = sm.remove_element(1, 0);
        assert!(ret.is_none());
        let ret = sm.remove_element(3, 2);
        assert_eq!(ret, Some(6));
        assert_eq!(sm.len(), 5);
    }
}