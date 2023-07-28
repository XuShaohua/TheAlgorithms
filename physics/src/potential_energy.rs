// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Finding the gravitational potential energy of an object with reference
//! to the earth,by taking its mass and height above the ground as input
//!
//! [Gravitational energy](https://en.m.wikipedia.org/wiki/Gravitational_energy)

const G: f64 = 9.80665;

#[must_use]
pub fn potential_energy(mass: f64, height: f64) -> f64 {
    debug_assert!(mass >= 0.0);
    debug_assert!(height >= 0.0);
    mass * G * height
}

#[cfg(test)]
mod tests {
    use super::potential_energy;

    #[test]
    fn test_potential_energy() {
        const PAIRS: &[(f64, f64, f64)] = &[
            (10.0, 10.0, 980.665),
            (0.0, 5.0, 0.0),
            (8.0, 0.0, 0.0),
            (10.0, 5.0, 490.3325),
            (0.0, 0.0, 0.0),
            (2.0, 8.0, 156.9064),
            (20.0, 100.0, 19613.3),
        ];
        for (mass, height, result) in PAIRS {
            assert_eq!(potential_energy(*mass, *height), *result);
        }
    }
}
