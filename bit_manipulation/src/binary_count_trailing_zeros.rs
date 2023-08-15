// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Take in 1 integer, return a number that is
//! the number of trailing zeros in binary representation of that number.

#[must_use]
pub fn count_trailing_zeros(a: i64) -> u32 {
    debug_assert!(a >= 0);
    if a == 0 {
        64
    } else {
        (a & -a).ilog2()
    }
}

#[inline]
#[must_use]
pub const fn builtin(a: i64) -> u32 {
    a.trailing_zeros()
}

#[cfg(test)]
mod tests {
    use super::{builtin, count_trailing_zeros};

    const PAIRS: &[(i64, u32)] = &[
        (25, 0),
        (36, 2),
        (16, 4),
        (58, 1),
        (4294967296, 32),
        (0, 64),
    ];

    #[test]
    fn test_count_trailing_zeros() {
        for (num, result) in PAIRS {
            assert_eq!(count_trailing_zeros(*num), *result);
        }
    }

    #[test]
    fn test_builtin() {
        for (num, result) in PAIRS {
            println!("num: {num}");
            assert_eq!(builtin(*num), *result);
        }
    }
}
