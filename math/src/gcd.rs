// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

//! Greatest Common Devisor ([GCD](https://en.wikipedia.org/wiki/Greatest_common_divisor))

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GcdError {
    InvalidValue,
}

/// [Euclidean algorithm](https://en.wikipedia.org/wiki/Euclidean_algorithm) is
/// an effective method for computing GCD.
///
/// # Errors
/// Returns error if `x` or `y` is not positive integer.
pub fn euclidean_recursive(x: i64, y: i64) -> Result<i64, GcdError> {
    if x < 0 || y < 0 {
        return Err(GcdError::InvalidValue);
    }
    let (x, y) = if x > y { (x, y) } else { (y, x) };
    if y == 0 {
        Ok(x)
    } else {
        euclidean_recursive(y, x % y)
    }
}
