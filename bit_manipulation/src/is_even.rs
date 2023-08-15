// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// return true if the input integer is even
///
/// Explanation: Lets take a look at the following deicmal to binary conversions
/// 2 => 10
/// 14 => 1110
/// 100 => 1100100
/// 3 => 11
/// 13 => 1101
/// 101 => 1100101
/// from the above examples we can observe that
/// for all the odd integers there is always 1 set bit at the end
/// also, 1 in binary can be represented as 001, 00001, or 0000001
/// so for any odd integer n => n&1 is always equals 1 else the integer is even
#[must_use]
pub const fn is_even(number: i32) -> bool {
    number & 1 == 0
}

#[cfg(test)]
mod tests {
    use super::is_even;

    #[test]
    fn test_is_even() {
        const PAIRS: &[(i32, bool)] = &[
            (1, false),
            (4, true),
            (9, false),
            (15, false),
            (40, true),
            (100, true),
            (101, false),
        ];
        for (num, result) in PAIRS {
            assert_eq!(is_even(*num), *result);
        }
    }
}
