// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! [floor and ceil](https://en.wikipedia.org/wiki/Floor_and_ceiling_functions)

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]

/// Return the floor of x as an integral.
#[must_use]
pub fn floor(x: f64) -> i64 {
    let x_i64 = x as i64;
    if x - x_i64 as f64 >= 0.0 {
        x_i64
    } else {
        x_i64 - 1
    }
}

/// Return the ceil of x as an integral.
#[must_use]
pub fn ceil(x: f64) -> i64 {
    let x_i64 = x as i64;
    if x - x_i64 as f64 > 0.0 {
        x_i64 + 1
    } else {
        x_i64
    }
}

#[cfg(test)]
mod tests {
    use super::{ceil, floor};

    #[test]
    fn test_floor() {
        for x in &[
            1.0_f64,
            -1.0,
            0.0,
            -0.0,
            1.1,
            -1.1,
            1.0,
            -1.0,
            1_000_000_000.0,
        ] {
            eprintln!("x: {x}");
            assert_eq!(floor(*x), x.floor() as i64);
        }
    }

    #[test]
    fn test_ceil() {
        for x in &[
            1.0_f64,
            -1.0,
            0.0,
            -0.0,
            1.1,
            -1.1,
            1.0,
            -1.0,
            1_000_000_000.0,
        ] {
            assert_eq!(ceil(*x), x.ceil() as i64);
        }
    }
}
