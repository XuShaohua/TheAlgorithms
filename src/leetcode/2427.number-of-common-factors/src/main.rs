// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(dead_code)]

#[must_use]
fn get_factors(num: i32) -> Vec<i32> {
    let mut factors = Vec::new();
    for i in 1..=num {
        if num % i == 0 {
            factors.push(i);
        }
    }
    factors
}

#[must_use]
fn get_factors_count(num: i32) -> i32 {
    let mut count: i32 = 0;
    for i in 1..=num {
        if num % i == 0 {
            count += 1;
        }
    }
    count
}

#[must_use]
const fn gcd(a: i32, b: i32) -> i32 {
    let (mut nom, mut denom) = if a > b { (a, b) } else { (b, a) };

    while denom != 0 {
        let rem: i32 = nom % denom;
        nom = denom;
        denom = rem;
    }
    nom
}

pub fn common_factors1(a: i32, b: i32) -> i32 {
    debug_assert!((1..=1000).contains(&a));
    debug_assert!((1..=1000).contains(&b));

    let common_divisor: i32 = gcd(a, b);
    get_factors_count(common_divisor)
}

pub type SolutionFn = fn(i32, i32) -> i32;

fn check_common_factors(func: SolutionFn) {
    assert_eq!(func(12, 6), 4);
    assert_eq!(func(25, 30), 2);
}

fn main() {
    check_common_factors(common_factors1);
}

#[cfg(test)]
mod tests {
    use super::{check_common_factors, common_factors1};

    #[test]
    fn test_common_factors1() {
        check_common_factors(common_factors1);
    }
}
