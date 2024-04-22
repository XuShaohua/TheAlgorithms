// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Given a positive int number, return True if this number is power of 2
/// or False otherwise.
///
/// Implementation notes: Use bit manipulation.
/// For example if the number is the power of two it's bits representation:
/// n     = 0..100..00
/// n - 1 = 0..011..11
///
/// n & (n - 1) - no intersections = 0
#[must_use]
pub const fn is_power_of_two(number: i32) -> bool {
    debug_assert!(number >= 0);
    number & (number - 1) == 0
}

#[cfg(test)]
mod tests {
    use super::is_power_of_two;

    #[test]
    fn test_is_power_of_two() {
        const PAIRS: &[(i32, bool)] = &[
            (0, true),
            (1, true),
            (2, true),
            (4, true),
            (6, false),
            (8, true),
            (10, false),
        ];

        for (num, result) in PAIRS {
            assert_eq!(is_power_of_two(*num), *result);
        }
    }

    #[test]
    fn test_is_power_of_two_all() {
        // Test all powers of 2 from 0 to 10,000
        assert!((0..20).all(|i| is_power_of_two(2_i32.pow(i))));
    }
}
