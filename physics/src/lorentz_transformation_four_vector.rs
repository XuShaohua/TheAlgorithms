// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by BSD-style License that can be found
// in the LICENSE file.

//! Lorentz transformations describe the transition between two inertial reference
//! frames F and F', each of which is moving in some direction with respect to the
//! other.
//!
//! [Lorentz transformation](https://en.wikipedia.org/wiki/Lorentz_transformation)

/// Speed of light (m/s)
pub const C: f64 = 299_792_458.0;

/// Vehicle's speed divided by speed of light (no units)
#[must_use]
pub fn beta(velocity: f64) -> f64 {
    debug_assert!(velocity <= C);
    debug_assert!(velocity >= 1.0);
    velocity / C
}

/// Calculate the Lorentz factor γ = 1 / √(1 - v²/c²) for a given velocity
#[must_use]
#[allow(clippy::suboptimal_flops)]
pub fn gamma(velocity: f64) -> f64 {
    1.0 / (1.0 - beta(velocity).powi(2)).sqrt()
}

/// Calculate the Lorentz transformation matrix for movement in the x direction:
#[must_use]
pub fn transformation_matrix(velocity: f64) -> [[f64; 4]; 4] {
    [
        [gamma(velocity), -gamma(velocity) * beta(velocity), 0.0, 0.0],
        [-gamma(velocity) * beta(velocity), gamma(velocity), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

/// Calculate a Lorentz transformation for movement in the x direction given a
/// velocity and a four-vector for an inertial reference frame
#[must_use]
pub fn transform(velocity: f64, event: &mut [f64; 4]) -> [f64; 4] {
    // Ensure event is not empty
    event[0] *= C; // x0 is ct (speed of light * time)

    let a = transformation_matrix(velocity);
    [
        a[0][0] * event[0],
        a[1][0].mul_add(event[0], event[1]),
        a[2][2] * event[2],
        a[3][3] * event[3],
    ]
}

#[cfg(test)]
mod tests {
    use super::{beta, gamma, transform, transformation_matrix, C};

    #[test]
    fn test_beta() {
        assert!((beta(C) - 1.0).abs() < f64::EPSILON);
        assert!((beta(199_792_458.0) - 0.666_435_904_801_848).abs() < f64::EPSILON);
        assert!((beta(1e5) - 0.000_333_564_095_198_152_05).abs() < f64::EPSILON);
    }

    #[test]
    fn test_gamma() {
        assert!((gamma(4.0) - 1.000_000_000_000_000_2).abs() < f64::EPSILON);
        assert!((gamma(1e5) - 1.000_000_055_632_507_5).abs() < f64::EPSILON);
        assert!((gamma(3e7) - 1.005_044_845_777_813).abs() < f64::EPSILON);
        assert!((gamma(2.8e8) - 2.798_559_572_231_827_7).abs() < f64::EPSILON);
        assert!((gamma(299_792_451.0) - 4_627.499_026_694_95).abs() < f64::EPSILON);
    }

    #[test]
    fn test_transformation_matrix() {
        assert_eq!(
            transformation_matrix(29_979_245.0),
            [
                [1.005_037_814_988_307_5, -0.100_503_778_816_874_5, 0.0, 0.0],
                [-0.100_503_778_816_874_5, 1.005_037_814_988_307_5, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        );
        assert_eq!(
            transformation_matrix(19_979_245.2),
            [
                [1.002_228_108_583_157, -0.066_792_077_630_302_22, 0.0, 0.0],
                [-0.066_792_077_630_302_22, 1.002_228_108_583_157, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        );
        assert_eq!(
            transformation_matrix(1.0),
            [
                [
                    1.000_000_00e+00,
                    -3.335_640_951_981_520_4e-9,
                    0.000_000_00e+00,
                    0.000_000_00e+00
                ],
                [
                    -3.335_640_951_981_520_4e-9,
                    1.000_000_00e+00,
                    0.000_000_00e+00,
                    0.000_000_00e+00
                ],
                [
                    0.000_000_00e+00,
                    0.000_000_00e+00,
                    1.000_000_00e+00,
                    0.000_000_00e+00
                ],
                [
                    0.000_000_00e+00,
                    0.000_000_00e+00,
                    0.000_000_00e+00,
                    1.000_000_00e+00
                ]
            ]
        );
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_transform() {
        assert_eq!(
            transform(29_979_245.0, &mut [1.0, 2.0, 3.0, 4.0]),
            [
                301_302_756.938_293_93,
                -30_130_272.889_799_137,
                3.000_000_00e+00,
                4.000_000_00e+00
            ]
        );
    }
}
