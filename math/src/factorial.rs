// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

//! The [Factorial](https://en.wikipedia.org/wiki/Factorial) of a non-negative
//! integer n, is the product of all postive integers less than or equal to n.

/// Calculate the factorial of specified number (n!).
#[must_use]
pub fn factorial(num: u8) -> u64 {
    let mut product = 1_u64;
    for i in 1..=u64::from(num) {
        product *= i;
    }

    product
}

#[cfg(test)]
mod tests {
    use super::factorial;

    #[test]
    fn test_factorial() {
        const PAIRS: &[(u8, u64)] = &[
            (0, 1),
            (1, 1),
            (6, 720),
            (10, 3_628_800),
            (15, 1_307_674_368_000),
            (20, 2_432_902_008_176_640_000),
        ];
        for (key, value) in PAIRS {
            assert_eq!(factorial(*key), *value);
        }
    }
}
