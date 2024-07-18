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
    #[allow(clippy::approx_constant)]
    fn test_get_radian() {
        const PAIRS: &[(f64, f64)] = &[
            (180.0, 3.141_592_653_589_793),
            (92.0, 1.605_702_911_834_783_2),
            (274.0, 4.782_202_150_464_463),
            (109.82, 1.916_720_584_540_172_5),
        ];

        for (degree, expected) in PAIRS.iter().copied() {
            let radian = get_radian(degree);
            assert!((radian - expected).abs() < f64::EPSILON);
        }
    }
}
