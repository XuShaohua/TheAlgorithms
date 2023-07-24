// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::error::BinaryError;

/// Converts from binary to hexadecimal
///
/// # Errors
/// Returns error if number is negative or contains non-binary digits.
pub const fn binary_to_hexadecimal(mut binary: i64) -> Result<i32, BinaryError> {
    if binary < 0 {
        return Err(BinaryError::OutOfRange);
    }

    let mut hexa = 0;
    let mut remainder;
    let mut i = 1;

    while binary > 0 {
        remainder = (binary % 10) as i32;
        if remainder > 1 {
            return Err(BinaryError::NonBinaryValue);
        }
        hexa += remainder * i;
        i *= 2;
        binary /= 10;
    }

    Ok(hexa)
}
