// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;

// Binary Search
pub fn is_perfect_square1(num: i32) -> bool {
    assert!(num >= 1);

    let mut left = 1;
    let mut right = num;
    let num64 = num as i64;

    while left <= right {
        let middle = left + (right - left) / 2;
        let square = middle as i64 * middle as i64;
        match square.cmp(&num64) {
            Ordering::Less => left = middle + 1,
            Ordering::Greater => right = middle - 1,
            Ordering::Equal => return true,
        }
    }
    false
}

pub type SolutionFn = fn(i32) -> bool;

fn check_solution(func: SolutionFn) {
    assert!(func(1));
    assert!(!func(8));
    assert!(func(9));
    assert!(!func(10));
    assert!(func(16));
    assert!(!func(i32::MAX));
}

fn main() {
    check_solution(is_perfect_square1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_perfect_square1};

    #[test]
    fn test_is_perfect_square1() {
        check_solution(is_perfect_square1);
    }
}
