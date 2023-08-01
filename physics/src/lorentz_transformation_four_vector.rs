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
        assert_eq!(beta(C), 1.0);
        assert_eq!(beta(199792458.0), 0.666435904801848);
        assert_eq!(beta(1e5), 0.00033356409519815205);
    }

    #[test]
    fn test_gamma() {
        assert_eq!(gamma(4.0), 1.0000000000000002);
        assert_eq!(gamma(1e5), 1.0000000556325075);
        assert_eq!(gamma(3e7), 1.005044845777813);
        assert_eq!(gamma(2.8e8), 2.7985595722318277);
        assert_eq!(gamma(299792451.0), 4627.49902669495);
    }

    #[test]
    fn test_transformation_matrix() {
        assert_eq!(
            transformation_matrix(29979245.0),
            [
                [1.0050378149883075, -0.1005037788168745, 0.0, 0.0],
                [-0.1005037788168745, 1.0050378149883075, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        );
        assert_eq!(
            transformation_matrix(19979245.2),
            [
                [1.002228108583157, -0.06679207763030222, 0.0, 0.0],
                [-0.06679207763030222, 1.002228108583157, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        );
        assert_eq!(
            transformation_matrix(1.0),
            [
                [
                    1.00000000e+00,
                    -3.3356409519815204e-9,
                    0.00000000e+00,
                    0.00000000e+00
                ],
                [
                    -3.3356409519815204e-9,
                    1.00000000e+00,
                    0.00000000e+00,
                    0.00000000e+00
                ],
                [
                    0.00000000e+00,
                    0.00000000e+00,
                    1.00000000e+00,
                    0.00000000e+00
                ],
                [
                    0.00000000e+00,
                    0.00000000e+00,
                    0.00000000e+00,
                    1.00000000e+00
                ]
            ]
        );
    }

    #[test]
    fn test_transform() {
        assert_eq!(
            transform(29979245.0, &mut [1.0, 2.0, 3.0, 4.0]),
            [
                301302756.93829393,
                -30130272.889799137,
                3.00000000e+00,
                4.00000000e+00
            ]
        );
    }
}
