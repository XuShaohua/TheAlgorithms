// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

//! Greatest Common Devisor ([GCD](https://en.wikipedia.org/wiki/Greatest_common_divisor))

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

#[cfg(test)]
mod tests {
    use super::{euclidean_iterative, euclidean_recursive};

    const PAIRS: &[(u64, u64, u64)] = &[
        (120, 7, 1),
        (312, 221, 13),
        (289, 204, 17),
        (1071, 462, 21),
        (147, 462, 21),
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
}
