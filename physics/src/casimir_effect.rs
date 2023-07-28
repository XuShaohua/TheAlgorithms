// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Finding the value of magnitude of either the Casimir force, the surface area
//! of one of the plates or distance between the plates provided that the other
//! two parameters are given.
//!
//! [Cassimir effect](https://en.wikipedia.org/wiki/Casimir_effect)

use std::f64::consts::PI;

const REDUCED_PLANCK_CONSTANT: f64 = 1.054_571_817e-34;
const SPEED_OF_LIGHT: f64 = 3e8;
const PI_2: f64 = PI * PI;

#[must_use]
pub fn get_area(distance: f64, force: f64) -> f64 {
    debug_assert!(force > 0.0);
    debug_assert!(distance > 0.0);

    (240.0 * force * distance.powi(4)) / (REDUCED_PLANCK_CONSTANT * SPEED_OF_LIGHT * PI_2)
}

#[must_use]
pub fn get_distance(area: f64, force: f64) -> f64 {
    debug_assert!(area > 0.0);
    debug_assert!(force > 0.0);

    (REDUCED_PLANCK_CONSTANT * SPEED_OF_LIGHT * PI_2 * area / (240.0 * force)).powf(1.0 / 4.0)
}

#[must_use]
pub fn get_force(area: f64, distance: f64) -> f64 {
    debug_assert!(area > 0.0);
    debug_assert!(distance > 0.0);

    (REDUCED_PLANCK_CONSTANT * SPEED_OF_LIGHT * PI_2 * area) / (240.0 * distance.powi(4))
}

#[cfg(test)]
mod tests {
    use super::{get_area, get_distance, get_force};

    #[test]
    fn test_get_area() {
        assert_eq!(get_area(0.0023746, 2737e-21,), 0.06688838837354052);
    }

    #[test]
    fn test_get_distance() {
        assert_eq!(get_distance(0.0023, 2635e-13), 1.0323056015031114e-05);
    }

    #[test]
    fn test_get_force() {
        assert_eq!(get_force(4.0, 0.03), 6.42481891748642e-21);
    }
}
