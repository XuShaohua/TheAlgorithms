// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Absolute value.

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::suboptimal_flops)]

/// Find the absolute value of a number.
#[must_use]
pub fn abs_val(num: f64) -> f64 {
    if num >= 0.0 {
        num
    } else {
        -num
    }
}

#[must_use]
pub const fn abs_val_i32(num: i32) -> i32 {
    if num >= 0 {
        num
    } else {
        -num
    }
}

#[must_use]
pub fn abs_min(list: &[i32]) -> i32 {
    debug_assert!(!list.is_empty());
    let mut num = list[0];
    for n in &list[1..] {
        if abs_val_i32(*n) < abs_val_i32(num) {
            num = *n;
        }
    }
    num
}

#[must_use]
pub fn abs_max(list: &[i32]) -> i32 {
    debug_assert!(!list.is_empty());
    let mut num = list[0];
    for n in &list[1..] {
        if abs_val_i32(*n) > abs_val_i32(num) {
            num = *n;
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::{abs_max, abs_min, abs_val};

    #[test]
    fn test_abs_val() {
        assert_eq!(abs_val(-5.1), 5.1);
        assert_eq!(abs_val(5.0), 5.0);
        assert_eq!(abs_val(0.0), 0.0);
    }

    #[test]
    fn test_abs_min() {
        assert_eq!(abs_min(&[0, 5, 1, 11]), 0);
        assert_eq!(abs_min(&[3, -10, -2]), -2);
    }

    #[test]
    fn test_abs_max() {
        assert_eq!(abs_max(&[0, 5, 1, 11]), 11);
        assert_eq!(abs_max(&[3, -10, -2]), -10);
    }
}
