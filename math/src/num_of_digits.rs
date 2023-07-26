// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

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

#[cfg(test)]
mod tests {
    use super::num_digits;

    #[test]
    fn test_num_digits() {
        const PAIRS: &[(i64, usize)] = &[(12345, 5), (123, 3), (0, 1), (-1, 1), (-123456, 6)];
        for (num, digits) in PAIRS {
            assert_eq!(num_digits(*num), *digits);
        }
    }
}
