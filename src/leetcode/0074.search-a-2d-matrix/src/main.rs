// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;

// Brute force
pub fn search_matrix1(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let items = matrix.len() * matrix[0].len();
    let mut nums = Vec::with_capacity(items);
    for row in matrix {
        nums.extend(row);
    }

    nums.binary_search(&target).is_ok()
}

// Binary Search
pub fn search_matrix2(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    // 两次二分查找:
    // 1. 确定 target 应该属于哪个行
    // 2. 确定 target 是否在当前行

    debug_assert!(!matrix.is_empty() && !matrix[0].is_empty());

    // 极端情况.
    let last_row = matrix.len() - 1;
    let last_col = matrix[0].len() - 1;
    if target < matrix[0][0] || target > matrix[last_row][last_col] {
        return false;
    }

    let mut top = 0;
    let mut bottom = last_row;
    // 循环终止条件是 top == bottom
    while top < bottom {
        let middle = top + (bottom - top + 1) / 2;
        if matrix[middle][0] > target {
            // target 位于 middle 上面的行
            bottom = middle - 1;
        } else {
            top = middle;
        }
    }
    debug_assert!(top == bottom);
    debug_assert!(matrix[top][0] <= target);
    if top < last_row {
        debug_assert!(target <= matrix[top + 1][0]);
    }

    let mut left = 0;
    let mut right = last_col;
    let row = &matrix[top];
    while left <= right {
        let middle = left + (right - left) / 2;
        match row[middle].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => left = middle + 1,
            Ordering::Greater => right = middle.saturating_sub(1),
        }
    }

    false
}

pub type SolutionFn = fn(Vec<Vec<i32>>, i32) -> bool;

fn check_solution(func: SolutionFn) {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert!(func(matrix, 3));

    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert!(!func(matrix, 13));

    let matrix = vec![vec![1], vec![3]];
    assert!(!func(matrix, 2));
}

fn main() {
    check_solution(search_matrix1);
    check_solution(search_matrix2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, search_matrix1, search_matrix2};

    #[test]
    fn test_search_matrix1() {
        check_solution(search_matrix1);
    }

    #[test]
    fn test_search_matrix2() {
        check_solution(search_matrix2);
    }
}
