// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Count the number of set bits in a 32 bit integer

#[must_use]
pub fn brian_kernighans_algorithm(mut number: i32) -> u32 {
    debug_assert!(number >= 0);
    let mut result = 0;
    while number != 0 {
        number &= number - 1;
        result += 1;
    }
    result
}

#[must_use]
pub fn modulo_operator(mut number: i32) -> u32 {
    debug_assert!(number >= 0);
    let mut result = 0;
    while number != 0 {
        if number % 2 == 1 {
            result += 1;
        }
        number >>= 1;
    }
    result
}

#[inline]
#[must_use]
pub const fn builtin(number: i32) -> u32 {
    number.count_ones()
}

#[cfg(test)]
mod tests {
    use super::{brian_kernighans_algorithm, builtin, modulo_operator};

    const PAIRS: &[(i32, u32)] = &[(25, 3), (37, 3), (21, 3), (58, 4), (0, 0), (256, 1)];

    #[test]
    fn test_brian_kernighans_algorithm() {
        for (num, result) in PAIRS {
            assert_eq!(brian_kernighans_algorithm(*num), *result);
        }
    }

    #[test]
    fn test_modulo_operator() {
        for (num, result) in PAIRS {
            assert_eq!(modulo_operator(*num), *result);
        }
    }

    #[test]
    fn test_builtin() {
        for (num, result) in PAIRS {
            assert_eq!(builtin(*num), *result);
        }
    }
}
