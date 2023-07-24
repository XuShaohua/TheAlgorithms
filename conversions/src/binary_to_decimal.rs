// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BinaryError {
    OutOfRange,
    NonBinaryValue,
}

/// Converts a number from [Binary to Decimal](https://en.wikipedia.org/wiki/Binary-coded_decimal).
///
/// The conversion is done by multiplying each digit of the binary number, starting
/// from the rightmost digit, with the power of 2 and adding the result.
///
/// # Errors
/// Returns error if number is negative or contains non-binary digits.
pub const fn binary_to_decimal(mut number: i64) -> Result<i32, BinaryError> {
    if number < 0 {
        return Err(BinaryError::OutOfRange);
    }

    let mut decimal_number = 0;
    let mut i = 0;
    while number > 0 {
        let digit = (number % 10) as i32;
        if digit > 1 {
            return Err(BinaryError::NonBinaryValue);
        }
        decimal_number += digit * 2_i32.pow(i);
        number /= 10;
        i += 1;
    }
    Ok(decimal_number)
}

#[cfg(test)]
mod tests {
    use super::{binary_to_decimal, BinaryError};

    #[test]
    fn test_binary_to_decimal() {
        assert_eq!(binary_to_decimal(111), Ok(7));
        assert_eq!(binary_to_decimal(101), Ok(5));
        assert_eq!(binary_to_decimal(1010), Ok(10));
        assert_eq!(binary_to_decimal(1101), Ok(13));
        assert_eq!(binary_to_decimal(100001), Ok(33));
        assert_eq!(binary_to_decimal(10101001), Ok(169));
        assert_eq!(binary_to_decimal(111010), Ok(58));
        assert_eq!(binary_to_decimal(100000000), Ok(256));
        assert_eq!(binary_to_decimal(10000000000), Ok(1024));
        assert_eq!(binary_to_decimal(101110111), Ok(375));
        assert_eq!(binary_to_decimal(-1010), Err(BinaryError::OutOfRange));
    }
}
