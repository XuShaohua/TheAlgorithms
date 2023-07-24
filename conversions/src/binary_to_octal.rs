// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::error::BinaryError;

/// Get the last three digits.
#[must_use]
const fn three_digits(mut num: i64) -> i32 {
    debug_assert!(num >= 0);

    let mut digits = 0;
    let mut p = 1;
    let mut remainder;
    let mut i = 0;

    while i < 3 {
        i += 1;
        remainder = (num % 10) as i32;
        digits += remainder * p;
        p *= 10;
        num /= 10;
    }
    digits
}

/// Convert binary number to octal number.
///
/// # Errors
/// Returns error if number is negative or contains non-binary digits.
#[allow(clippy::cast_possible_truncation)]
pub const fn binary_to_octal(mut binary_num: i64) -> Result<i32, BinaryError> {
    if binary_num < 0 {
        return Err(BinaryError::OutOfRange);
    }
    let mut octal: i32 = 0;
    let mut td: i32;
    let mut ord: i32 = 1;

    while binary_num > 0 {
        if binary_num > 111 {
            td = three_digits(binary_num);
        } else {
            td = binary_num as i32;
        }

        binary_num /= 1000;

        // Converting the last three digits to decimal.
        let mut d = 0;
        let mut base = 1;
        let mut remainder;
        while td > 0 {
            remainder = td % 10;
            if remainder > 1 {
                return Err(BinaryError::NonBinaryValue);
            }
            td /= 10;
            d += remainder * base;
            base *= 2;
        }

        octal += d * ord;
        ord *= 10;
    }

    Ok(octal)
}
