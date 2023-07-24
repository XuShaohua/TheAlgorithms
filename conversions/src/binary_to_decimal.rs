// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Converts a number from [Binary to Decimal](https://en.wikipedia.org/wiki/Binary-coded_decimal).
///
/// The conversion is done by multiplying each digit of the binary number, starting
/// from the rightmost digit, with the power of 2 and adding the result.
#[must_use]
pub const fn binary_to_decimal(mut number: i64) -> i32 {
    debug_assert!(number >= 0);

    let mut decimal_number = 0;
    let mut i = 0;
    while number > 0 {
        let digit = (number % 10) as i32;
        decimal_number += digit * 2_i32.pow(i);
        number /= 10;
        i += 1;
    }
    decimal_number
}

#[cfg(test)]
mod tests {
    use super::binary_to_decimal;

    #[test]
    fn test_binary_to_decimal() {
        assert_eq!(binary_to_decimal(111), 7);
        assert_eq!(binary_to_decimal(101), 5);
        assert_eq!(binary_to_decimal(1010), 10);
        assert_eq!(binary_to_decimal(1101), 13);
        assert_eq!(binary_to_decimal(100001), 33);
        assert_eq!(binary_to_decimal(10101001), 169);
        assert_eq!(binary_to_decimal(111010), 58);
        assert_eq!(binary_to_decimal(100000000), 256);
        assert_eq!(binary_to_decimal(10000000000), 1024);
        assert_eq!(binary_to_decimal(101110111), 375);
    }
}
