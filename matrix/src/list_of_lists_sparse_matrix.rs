// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::linkedlist)]
#![allow(dead_code)]

use std::collections::LinkedList;
use std::fmt;

use crate::traits::IsZero;

#[derive(Debug)]
pub struct ListOfListsSparseMatrix<T: IsZero + fmt::Debug> {
    rows: LinkedList<Row<T>>,
    len: usize,
}

/// Row number in list is ordered ascending.
#[derive(Debug)]
pub struct Row<T: fmt::Debug> {
    row: usize,
    columns: LinkedList<Column<T>>,
}

/// Column number in list is ordered ascending.
#[derive(Debug)]
pub struct Column<T: fmt::Debug> {
    column: usize,
    value: T,
}

impl<T: IsZero + fmt::Debug> ListOfListsSparseMatrix<T> {
    #[must_use]
    pub fn construct<I, I2>(sparse_matrix: I) -> Self
    where
        I: IntoIterator<Item=I2>,
        I2: IntoIterator<Item=T>,
    {
        let mut row_list = LinkedList::new();
        let mut len = 0;

        for (row, rows) in sparse_matrix.into_iter().enumerate() {
            let mut column_list = LinkedList::new();
            for (column, element) in rows.into_iter().enumerate() {
                if element.is_not_zero() {
                    column_list.push_back(Column { column, value: element });
                }
            }
            if !column_list.is_empty() {
                len += column_list.len();
                row_list.push_back(Row { row, columns: column_list });
            }
        }
        Self { rows: row_list, len }
    }

    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Get node value at (row, column).
    #[must_use]
    pub fn value(&self, row: usize, column: usize) -> Option<T> {
        for row_list in &self.rows {
            if row_list.row == row {
                for column_element in &row_list.columns {
                    if column_element.column == column {
                        return Some(column_element.value);
                    }
                }
                break;
            }
        }
        None
    }

    /// Get mutable reference to node value at (row, column).
    #[must_use]
    pub fn value_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        for row_list in &mut self.rows {
            if row_list.row == row {
                for column_element in &mut row_list.columns {
                    if column_element.column == column {
                        return Some(&mut column_element.value);
                    }
                }
                break;
            }
        }
        None
    }

    /// If found old node at (row, column), returns old value; otherwise returns None.
    #[allow(dead_code)]
    pub fn add_element(&self, _ow: usize, _column: usize, _value: T) -> Option<T> {
        // 1. Find the element at (row, column)
        // 2. If no columns_list found in rows, add a new one
        // 3. Add that element to selected column_list
        todo!()
        // 1. if rows list if empty, push to back
        // 2. if front
    }

    /// If found node at (row, column), returns value of that node; otherwise returns None.
    pub fn remove_element(&mut self, row: usize, column: usize) -> Option<T> {
        // 1. Find the element at (row, column)
        // 2. Remove the element in columns list
        // 3. If the columns list is empty, remove it from rows list

        let mut value = None;
        let mut row_index = 0;
        let mut remove_column_list = false;
        for row_list in &mut self.rows {
            row_index += 1;
            if row_list.row == row {
                for column_element in &mut row_list.columns {
                    if column_element.column == column {
                        value = Some(column_element.value);
                        break;
                    }
                }

                if row_list.columns.is_empty() && value.is_some() {
                    remove_column_list = true;
                }

                break;
            }
        }

        if remove_column_list {
            let mut tail = self.rows.split_off(row_index);
            // Remove that columns list.
            tail.pop_front();
            // Then merge together again.
            if !tail.is_empty() {
                self.rows.append(&mut tail);
            }
        }
        if value.is_some() {
            self.len -= 1;
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use crate::list_of_lists_sparse_matrix::ListOfListsSparseMatrix;

    #[test]
    fn test_array() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0]
        ];
        let sm = ListOfListsSparseMatrix::construct(MATRIX);
        println!("sm: {sm:?}");
        assert!(!sm.is_empty());
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
        let sm = ListOfListsSparseMatrix::construct(matrix);
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
        let sm = ListOfListsSparseMatrix::construct(matrix);
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
        let sm = ListOfListsSparseMatrix::construct(MATRIX);
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
        let mut sm = ListOfListsSparseMatrix::construct(MATRIX);
        let ret = sm.value_mut(0, 2);
        assert!(ret.is_some());
        if let Some(value) = ret {
            *value *= 2;
        }
        assert_eq!(sm.value(0, 2), Some(6));
    }

    // #[test]
    // fn test_add_element() {
    //     const MATRIX: [[i32; 5]; 4] = [
    //         [0, 0, 3, 0, 4],
    //         [0, 0, 5, 7, 0],
    //         [0, 0, 0, 0, 0],
    //         [0, 2, 6, 0, 0]
    //     ];
    //     let mut sm = ListOfListsSparseMatrix::construct(MATRIX);
    //     sm.add_element(1, 0, 1);
    //     assert_eq!(sm.len(), 7);
    //     assert_eq!(sm.value(1, 0), Some(1));
    // }

    #[test]
    fn test_remove_element() {
        const MATRIX: [[i32; 5]; 4] = [
            [0, 0, 3, 0, 4],
            [0, 0, 5, 7, 0],
            [0, 0, 0, 0, 0],
            [0, 2, 6, 0, 0]
        ];
        let mut sm = ListOfListsSparseMatrix::construct(MATRIX);
        let ret = sm.remove_element(1, 0);
        assert!(ret.is_none());
        let ret = sm.remove_element(3, 2);
        assert_eq!(ret, Some(6));
        assert_eq!(sm.len(), 5);
    }
}