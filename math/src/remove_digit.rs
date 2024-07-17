// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::digits::get_digits;

/// Returns the biggest possible result that can be achieved by removing
/// one digit from the given number
#[must_use]
pub fn remove_digit(num: i64) -> u64 {
    let digits: Vec<u8> = get_digits(num);
    let num_from_digits = |digits: &[u8], skip: usize| -> u64 {
        let mut product = 0;
        for (index, digit) in digits.iter().enumerate() {
            if index != skip {
                product = product * 10 + u64::from(*digit);
            }
        }
        product
    };

    let len = digits.len();
    (0..len)
        .map(|i| num_from_digits(&digits, i))
        .max()
        .unwrap_or_default()
}

/*
pub const fn remove_digit_fast(num: i64) -> i64 {
    let num_str = num.abs().to_string();
    let num_transpositions = [list(num_str) for char in range(len(num_str))]
    for index in 0..num_str.len() {
        num_transpositions[index].pop(index)
    }
    return max( int("".join(list(transposition))) for transposition in num_transpositions)
}
*/

#[cfg(test)]
mod tests {
    use super::remove_digit;

    const PAIRS: &[(i64, u64)] = &[(152, 52), (6385, 685), (-11, 1), (222_222, 22_222)];

    #[test]
    fn test_remove_digit() {
        for (key, val) in PAIRS {
            assert_eq!(remove_digit(*key), *val);
        }
    }
}
