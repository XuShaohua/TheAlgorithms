// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Binary search
pub fn nth_ugly_number1(n: i32, a: i32, b: i32, c: i32) -> i32 {
    #[must_use]
    const fn gcd(a: i64, b: i64) -> i64 {
        let (mut nom, mut denom) = if a > b { (a, b) } else { (b, a) };

        while denom != 0 {
            let rem: i64 = nom % denom;
            nom = denom;
            denom = rem;
        }
        nom
    }
    debug_assert!(a >= 1 && b >= 1 && c >= 1);

    let a: i64 = a as i64;
    let b: i64 = b as i64;
    let c: i64 = c as i64;
    let ab: i64 = a * b / gcd(a, b);
    let bc: i64 = b * c / gcd(b, c);
    let ac: i64 = a * c / gcd(a, c);
    let abc: i64 = a * bc / gcd(a, bc);
    let n: i64 = n as i64;

    let mut low: i64 = 1;
    let mut high: i64 = 2_000_000_000;
    while low < high {
        let middle: i64 = low + (high - low) / 2;
        let count: i64 =
            middle / a + middle / b + middle / c - middle / ab - middle / bc - middle / ac
                + middle / abc;
        if count < n {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    low as i32
}

pub type SolutionFn = fn(i32, i32, i32, i32) -> i32;

fn check_nth_ugly_number(func: SolutionFn) {
    assert_eq!(func(3, 2, 3, 5), 4);
    assert_eq!(func(4, 2, 3, 4), 6);
    assert_eq!(func(5, 2, 11, 13), 10);
}

fn main() {
    check_nth_ugly_number(nth_ugly_number1);
}

#[cfg(test)]
mod tests {
    use super::{check_nth_ugly_number, nth_ugly_number1};

    #[test]
    fn test_nth_ugly_number1() {
        check_nth_ugly_number(nth_ugly_number1);
    }
}
