// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::collections::HashSet;

// Brute force
pub fn search_matrix1(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let items = matrix.len() * matrix[0].len();
    let mut list = Vec::with_capacity(items);
    for row in matrix {
        list.extend(row);
    }

    list.sort_unstable();

    list.binary_search(&target).is_ok()
}

// Brute force
pub fn search_matrix2(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut set = HashSet::new();
    for row in matrix {
        set.extend(row);
    }

    set.contains(&target)
}

pub fn search_matrix3(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    debug_assert!(!matrix.is_empty());
    debug_assert!(!matrix[0].is_empty());

    // 从右上角开始找
    // 如果当前元素比 target 大, 就向左找更小的元素
    // 如果当前元素比 target 小, 就向下找更大的元素
    // 如果找完了所有空间, 还没找到, 就说明不存在
    let rows = matrix.len();
    let cols = matrix[0].len();

    // 从右上角开始找
    let mut row_index = 0;
    let mut col_index = cols - 1;

    // 循环终止的条件是达到左下角
    while row_index < rows {
        match matrix[row_index][col_index].cmp(&target) {
            Ordering::Equal => return true,
            // 向下找
            Ordering::Less => row_index += 1,
            // 向左找
            Ordering::Greater => {
                if col_index == 0 {
                    break;
                } else {
                    col_index -= 1;
                }
            }
        }
    }

    false
}

pub type SolutionFn = fn(Vec<Vec<i32>>, i32) -> bool;

fn check_solution(func: SolutionFn) {
    let matrix = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    assert!(func(matrix, 5));

    let matrix = vec![
        vec![1, 4, 7, 11, 15],
        vec![2, 5, 8, 12, 19],
        vec![3, 6, 9, 16, 22],
        vec![10, 13, 14, 17, 24],
        vec![18, 21, 23, 26, 30],
    ];
    assert!(!func(matrix, 20));

    let matrix = vec![vec![-5]];
    assert!(func(matrix, -5));

    let matrix = vec![vec![-1, 3]];
    assert!(func(matrix, 3));

    let matrix = vec![vec![-1, 3]];
    assert!(func(matrix, -1));
}

fn main() {
    check_solution(search_matrix1);
    check_solution(search_matrix2);
    check_solution(search_matrix3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, search_matrix1, search_matrix2, search_matrix3};

    #[test]
    fn test_search_matrix1() {
        check_solution(search_matrix1);
    }

    #[test]
    fn test_search_matrix2() {
        check_solution(search_matrix2);
    }

    #[test]
    fn test_search_matrix3() {
        check_solution(search_matrix3);
    }
}
