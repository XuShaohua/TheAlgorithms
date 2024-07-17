// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]

/// Find the number of digits in a number.
#[must_use]
pub const fn num_digits(num: i64) -> usize {
    let mut digits = 0;
    let mut n = num.abs();
    loop {
        n /= 10;
        digits += 1;
        if n == 0 {
            break;
        }
    }
    digits
}

/// Find the number of digits in a number.
///
/// `abs()` is used as logarithm for negative numbers is not defined.
#[must_use]
pub fn num_digits_fast(num: i64) -> usize {
    if num == 0 {
        1
    } else {
        let num = num.abs() as f64;
        (num.log10() + 1.0).floor() as usize
    }
}

/// Find the number of digits in a number.
///
/// `abs()` is used for negative numbers
#[must_use]
pub fn num_digits_faster(num: i64) -> usize {
    num.abs().to_string().len()
}

#[cfg(test)]
mod tests {
    use super::{num_digits, num_digits_fast, num_digits_faster};

    const PAIRS: &[(i64, usize)] = &[(12345, 5), (123, 3), (0, 1), (-1, 1), (-123_456, 6)];

    #[test]
    fn test_num_digits() {
        for (num, digits) in PAIRS {
            assert_eq!(num_digits(*num), *digits);
        }
    }

    #[test]
    fn test_num_digits_fast() {
        for (num, digits) in PAIRS {
            assert_eq!(num_digits_fast(*num), *digits);
        }
    }

    #[test]
    fn test_num_digits_faster() {
        for (num, digits) in PAIRS {
            assert_eq!(num_digits_faster(*num), *digits);
        }
    }
}
