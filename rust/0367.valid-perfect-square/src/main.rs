// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn is_perfect_square(num: i32) -> bool {
    use std::cmp::Ordering;

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

fn check_solution() {
    assert!(is_perfect_square(1));
    assert!(!is_perfect_square(8));
    assert!(is_perfect_square(9));
    assert!(!is_perfect_square(10));
    assert!(!is_perfect_square(14));
    assert!(is_perfect_square(16));
    assert!(!is_perfect_square(i32::MAX));
}

fn main() {
    check_solution();
}
