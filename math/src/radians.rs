// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Coverts the given angle from degrees to radians
//!
//! [Radian](https://en.wikipedia.org/wiki/Radian)

use std::f64::consts::PI;

#[must_use]
pub fn get_radian(degree: f64) -> f64 {
    degree / (180.0 / PI)
}

#[cfg(test)]
mod tests {
    use super::get_radian;

    #[test]
    fn test_get_radian() {
        const PAIRS: &[(f64, f64)] = &[
            (180.0, 3.141592653589793),
            (92.0, 1.6057029118347832),
            (274.0, 4.782202150464463),
            (109.82, 1.9167205845401725),
        ];

        for (degree, radian) in PAIRS {
            assert_eq!(get_radian(*degree), *radian);
        }
    }
}
