// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub type SolutionFn = fn(i32) -> i32;

pub fn my_sqrt(x: i32) -> i32 {
    if x == 0 || x == 1 {
        return x;
    }

    let mut left = 0;
    let mut right = x;
    let mut middle: i32;
    let mut ans = 0;
    let x1 = x as i64;

    while left <= right {
        middle = left + (right - left) / 2;
        let square = (middle as i64) * (middle as i64);
        if square > x1 {
            right = middle - 1;
        } else {
            ans = middle;
            left = middle + 1;
        }
    }
    ans
}

pub fn my_sqrt2(x: i32) -> i32 {
    let x1 = x as i64;
    let mut left = 0;
    let mut right = x1;

    while left < right {
        let middle = left + (right - left + 1) / 2;
        let square = middle * middle;
        if square > x1 {
            right = middle - 1;
        } else {
            left = middle;
        }
    }
    left as i32
}

pub fn my_sqrt3(x: i32) -> i32 {
    use std::cmp::Ordering;

    let mut left = 1;
    let mut right = x;

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
    check_solution(my_sqrt);
    check_solution(my_sqrt2);
    check_solution(my_sqrt3);
    println!("END");
}
