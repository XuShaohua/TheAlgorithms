// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Greatest Common Devisor ([GCD](https://en.wikipedia.org/wiki/Greatest_common_divisor))

use crate::factors::get_factors;

/// [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm) is
/// an effective method for computing GCD.
#[must_use]
pub const fn euclidean_recursive(x: u64, y: u64) -> u64 {
    let (x, y) = if x > y { (x, y) } else { (y, x) };
    if y == 0 {
        x
    } else {
        euclidean_recursive(y, x % y)
    }
}

#[must_use]
pub const fn euclidean_iterative(x: u64, y: u64) -> u64 {
    let (mut base_num, mut previous_remainder) = if x > y { (x, y) } else { (y, x) };
    if previous_remainder == 0 {
        return previous_remainder;
    }

    while base_num % previous_remainder != 0 {
        let old_base = base_num;
        base_num = previous_remainder;
        previous_remainder = old_base % previous_remainder;
    }

    previous_remainder
}

/// Get gcd by factors.
#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub fn by_factors(x: u64, y: u64) -> u64 {
    let x_factors = get_factors(x);
    let y_factors = get_factors(y);
    let mut gcd = 1;
    for (x_factor, x_count) in &x_factors {
        if y_factors.contains_key(x_factor) {
            let count = y_factors[x_factor].min(*x_count) as u32;
            gcd *= x_factor.pow(count);
        }
    }
    gcd
}

#[cfg(test)]
mod tests {
    use super::{by_factors, euclidean_iterative, euclidean_recursive};

    const PAIRS: &[(u64, u64, u64)] = &[
        (120, 7, 1),
        (312, 221, 13),
        (289, 204, 17),
        (1071, 462, 21),
        (147, 462, 21),
        (2520, 8350, 10),
    ];

    #[test]
    fn test_euclidean_recursive() {
        for (x, y, z) in PAIRS {
            assert_eq!(euclidean_recursive(*x, *y), *z);
        }
    }

    #[test]
    fn test_euclidean_iterative() {
        for (x, y, z) in PAIRS {
            assert_eq!(euclidean_iterative(*x, *y), *z);
        }
    }

    #[test]
    fn test_by_factors() {
        for (x, y, z) in PAIRS {
            assert_eq!(by_factors(*x, *y), *z);
        }
    }
}
