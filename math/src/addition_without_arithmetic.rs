// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Impl addition operation with bitwise operation.
//!
//! [Bitwise operation](https://en.wikipedia.org/wiki/Bitwise_operation)

#[must_use]
pub const fn add(mut first: i64, mut second: i64) -> i64 {
    while second != 0 {
        let c = first & second;
        first ^= second;
        second = c << 1;
    }
    first
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn test_add() {
        const PAIRS: &[(i64, i64, i64)] = &[
            (3, 5, 8),
            (13, 5, 18),
            (-7, 2, -5),
            (0, -7, -7),
            (-321, 0, -321),
        ];
        for (first, second, sum) in PAIRS {
            assert_eq!(add(*first, *second), *sum);
        }
    }
}
