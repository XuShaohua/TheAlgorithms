// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashSet;

// HashTable
// 使用 HashSet 来计录数值是否被计算过.
pub fn is_happy1(n: i32) -> bool {
    assert!(n >= 1);

    #[must_use]
    #[inline(always)]
    fn sum_digits(mut n: i32) -> i32 {
        let mut sum: i32 = 0;

        while n > 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        sum
    }

    let mut cache: HashSet<i32> = HashSet::new();

    let mut n: i32 = n;
    loop {
        n = sum_digits(n);
        if n == 1 {
            return true;
        }
        if cache.contains(&n) {
            return false;
        }
        cache.insert(n);
    }
}

pub type SolutionFn = fn(i32) -> bool;

fn check_solution(func: SolutionFn) {
    assert!(func(19));
    assert!(!func(2));
}

fn main() {
    check_solution(is_happy1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_happy1};

    #[test]
    fn test_is_happy1() {
        check_solution(is_happy1);
    }
}
