// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;

// Binary Search
pub fn my_sqrt1(x: i32) -> i32 {
    assert!(x >= 0);

    if x == 0 || x == 1 {
        return x;
    }

    let mut left: i32 = 0;
    let mut right: i32 = x;
    let mut ans: i32 = 0;

    while left <= right {
        let middle: i32 = left + (right - left) / 2;
        let square = middle.saturating_mul(middle);
        if square > x {
            // 值太大了, 右侧的值向左移
            right = middle - 1;
        } else {
            ans = middle;
            // 有些小, 左侧的值向右移
            left = middle + 1;
        }
    }
    ans
}

// Binary Search
pub fn my_sqrt2(x: i32) -> i32 {
    let mut left = 0;
    let mut right = x;

    while left < right {
        let middle = left + (right - left + 1) / 2;
        let square = middle.saturating_mul(middle);
        if square > x {
            right = middle - 1;
        } else {
            // 注意这里的边界情况.
            left = middle;
        }
    }
    left
}

pub fn my_sqrt3(x: i32) -> i32 {
    assert!(x >= 0);

    if x == 0 || x == 1 {
        return x;
    }

    let mut left: i32 = 0;
    let mut right: i32 = x;

    while left <= right {
        let middle = left + (right - left) / 2;
        match middle.cmp(&(x / middle)) {
            Ordering::Less => left = middle + 1,
            Ordering::Greater => right = middle - 1,
            Ordering::Equal => return middle,
        }
    }
    right
}

pub type SolutionFn = fn(i32) -> i32;

fn check_solution(func: SolutionFn) {
    assert_eq!(func(36), 6);
    assert_eq!(func(4), 2);
    assert_eq!(func(10), 3);
    assert_eq!(func(8), 2);
    assert_eq!(func(9), 3);
    assert_eq!(func(0), 0);
    assert_eq!(func(i32::MAX), 46340);
    assert_eq!(func(1), 1);
    assert_eq!(func(2), 1);
    assert_eq!(func(3), 1);
}

fn main() {
    check_solution(my_sqrt1);
    check_solution(my_sqrt2);
    check_solution(my_sqrt3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, my_sqrt1, my_sqrt2, my_sqrt3};

    #[test]
    fn test_my_sqrt1() {
        check_solution(my_sqrt1);
    }

    #[test]
    fn test_my_sqrt2() {
        check_solution(my_sqrt2);
    }

    #[test]
    fn test_my_sqrt3() {
        check_solution(my_sqrt3);
    }
}
