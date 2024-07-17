// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Find the kinetic energy of an object, given its mass and velocity.
//!
//! [Kinetic energy](https://en.m.wikipedia.org/wiki/Kinetic_energy)

/// Calculate kinetic energy.
///
/// The kinetic energy of a non-rotating object of mass m traveling at a speed v is ½mv²
#[must_use]
pub fn kinetic_energy(mass: f64, velocity: f64) -> f64 {
    debug_assert!(mass >= 0.0);
    0.5 * mass * velocity.abs() * velocity.abs()
}

#[cfg(test)]
mod tests {
    use super::kinetic_energy;

    #[test]
    fn test_kinetic_energy() {
        const PAIRS: &[(f64, f64, f64)] = &[
            (10.0, 10.0, 500.0),
            (0.0, 10.0, 0.0),
            (10.0, 0.0, 0.0),
            (20.0, -20.0, 4000.0),
            (0.0, 0.0, 0.0),
            (2.0, 2.0, 4.0),
            (100.0, 100.0, 500_000.0),
        ];

        for (mass, velocity, result) in PAIRS {
            assert!((kinetic_energy(*mass, *velocity) - result).abs() < f64::EPSILON);
        }
    }
}
