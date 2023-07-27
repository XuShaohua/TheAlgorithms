// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::ops::{Div, Rem};

const ROMAN: &[(u32, &str)] = &[
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

const ROMAN_INVERSE: &[(u8, u32)] = &[
    (b'I', 1),
    (b'V', 5),
    (b'X', 10),
    (b'L', 50),
    (b'C', 100),
    (b'D', 500),
    (b'M', 1000),
];

#[allow(clippy::cast_possible_truncation)]
fn to_int(byte: u8) -> u32 {
    for (k, v) in ROMAN_INVERSE {
        if k == &byte {
            return *v;
        }
    }
    0
}

/// Given a roman numeral, convert it to an integer.
///
/// [Roman Numerals](https://en.wikipedia.org/wiki/Roman_numerals)
#[must_use]
pub fn roman_to_int(roman: &str) -> u32 {
    let mut total = 0;
    let mut place = 0;
    let len = roman.len();
    while place < len {
        let p0 = to_int(roman.as_bytes()[place]);
        if place + 1 < len {
            let p1 = to_int(roman.as_bytes()[place + 1]);
            if p0 < p1 {
                total += p1 - p0;
                place += 2;
                continue;
            }
        }
        total += p0;
        place += 1;
    }
    total
}

/// Given a integer, convert it to an roman numeral.
///
/// [Roman Numerals](https://en.wikipedia.org/wiki/Roman_numerals)
#[must_use]
pub fn int_to_roman(mut number: u32) -> String {
    let mut result = String::new();
    for (arabic, roman) in ROMAN {
        let factor = number.div(arabic);
        number = number.rem(arabic);
        for _i in 0..factor {
            result.push_str(roman);
        }
        if number == 0 {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::{int_to_roman, roman_to_int};

    const PAIRS: &[(&str, u32)] = &[
        ("III", 3),
        ("CLIV", 154),
        ("MIX", 1009),
        ("MMD", 2500),
        ("MMMCMXCIX", 3999),
    ];

    #[test]
    fn test_roman_to_int() {
        for (s, val) in PAIRS {
            assert_eq!(roman_to_int(s), *val);
        }
    }

    #[test]
    fn test_int_to_roman() {
        for (s, val) in PAIRS {
            assert_eq!(&int_to_roman(*val), s);
        }
    }
}
