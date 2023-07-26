// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::cast_sign_loss, clippy::module_name_repetitions)]

/// Find the number of digits in a number.
#[must_use]
pub fn get_digits(num: i64) -> Vec<u8> {
    let mut digits = vec![];
    let mut n = num.abs();
    loop {
        let digit = (n % 10) as u8;
        n /= 10;
        digits.push(digit);
        if n == 0 {
            break;
        }
    }
    digits.reverse();
    digits
}

#[cfg(test)]
mod tests {
    use super::get_digits;

    const PAIRS: &[(i64, &[u8])] = &[
        (12345, &[1, 2, 3, 4, 5]),
        (123, &[1, 2, 3]),
        (0, &[0]),
        (-1, &[1]),
        (-123456, &[1, 2, 3, 4, 5, 6]),
    ];

    #[test]
    fn test_get_digits() {
        for (key, val) in PAIRS {
            assert_eq!(&get_digits(*key), val);
        }
    }
}
