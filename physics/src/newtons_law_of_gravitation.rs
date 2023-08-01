// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Finding the value of either Gravitational Force, one of the masses or distance
//! provided that the other three parameters are given.
//!
//! [Newton's law of universal gravitation](https://en.wikipedia.org/wiki/Newton%27s_law_of_universal_gravitation)

/// Define the Gravitational Constant G and the function
///
/// unit of G : m^3 * kg^-1 * s^-2
pub const GRAVITATIONAL_CONSTANT: f64 = 6.6743e-11;

#[must_use]
pub fn get_force(mass_1: f64, mass_2: f64, distance: f64) -> f64 {
    debug_assert!(mass_1 >= 0.0);
    debug_assert!(mass_2 >= 0.0);
    debug_assert!(distance >= 0.0);
    GRAVITATIONAL_CONSTANT * mass_1 * mass_2 / (distance.powi(2))
}

#[must_use]
pub fn get_mass_1(force: f64, mass_2: f64, distance: f64) -> f64 {
    debug_assert!(force >= 0.0);
    debug_assert!(mass_2 >= 0.0);
    debug_assert!(distance >= 0.0);
    force * distance.powi(2) / (GRAVITATIONAL_CONSTANT * mass_2)
}

#[must_use]
pub fn get_mass_2(force: f64, mass_1: f64, distance: f64) -> f64 {
    debug_assert!(force >= 0.0);
    debug_assert!(mass_1 >= 0.0);
    debug_assert!(distance >= 0.0);
    force * (distance.powi(2)) / (GRAVITATIONAL_CONSTANT * mass_1)
}

#[must_use]
pub fn get_distance(force: f64, mass_1: f64, mass_2: f64) -> f64 {
    debug_assert!(force >= 0.0);
    debug_assert!(mass_1 >= 0.0);
    debug_assert!(mass_2 >= 0.0);
    (GRAVITATIONAL_CONSTANT * mass_1 * mass_2 / force).sqrt()
}
