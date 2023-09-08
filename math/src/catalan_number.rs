// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Calculate the nth Catalan number
//!
//! [Catalan number](https://en.wikipedia.org/wiki/Catalan_number)

/// Get nth catalan number.
#[must_use]
#[allow(clippy::cast_possible_wrap)]
pub fn catalan(number: usize) -> i64 {
    debug_assert!(number > 0);
    let mut current_number = 1;

    for i in 1..(number as i64) {
        current_number *= 4 * i - 2;
        current_number /= i + 1;
    }

    current_number
}

#[cfg(test)]
mod tests {
    use super::catalan;

    #[test]
    fn test_catalan() {
        assert_eq!(catalan(5), 14);
    }
}
