// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Coulomb's Law states that the magnitude of the electrostatic force of
//! attraction or repulsion between two point charges is directly proportional
//! to the product of the magnitudes of charges and inversely proportional to
//! the square of the distance between them.
//!
//! Apply Coulomb's Law on any three given values.
//!
//! [Coulomb's law](https://en.wikipedia.org/wiki/Coulomb%27s_law)
//! Coulomb (1785) "Premier mémoire sur l’électricité et le magnétisme,"
//!  Histoire de l’Académie Royale des Sciences, pp. 569–577.

// units = N * m^s * C^-2
pub const COULOMBS_CONSTANT: f64 = 8.988e9;

/// # Parameters
/// - `force` - units in Newtons
/// - `charge1` - units in Coulombs
/// - `charge2` - units in Coulombs
/// - `distance` - units in meters
#[must_use]
pub fn get_force(charge1: f64, charge2: f64, distance: f64) -> f64 {
    debug_assert!(distance >= 0.0);

    let charge_product = (charge1 * charge2).abs();
    COULOMBS_CONSTANT * charge_product / (distance.powi(2))
}

#[must_use]
#[allow(clippy::suspicious_operation_groupings)]
pub fn get_charge1(force: f64, charge2: f64, distance: f64) -> f64 {
    debug_assert!(distance >= 0.0);

    force.abs() * distance.powi(2) / (COULOMBS_CONSTANT * charge2)
}

#[must_use]
#[allow(clippy::suspicious_operation_groupings)]
pub fn get_charge2(force: f64, charge1: f64, distance: f64) -> f64 {
    debug_assert!(distance >= 0.0);

    force.abs() * distance.powi(2) / (COULOMBS_CONSTANT * charge1)
}

#[must_use]
pub fn get_distance(force: f64, charge1: f64, charge2: f64) -> f64 {
    let charge_product = (charge1 * charge2).abs();
    (COULOMBS_CONSTANT * charge_product / force.abs()).sqrt()
}

#[cfg(test)]
mod tests {
    use super::{get_charge1, get_distance, get_force};

    #[test]
    fn test_get_force() {
        assert_eq!(get_force(3.0, 5.0, 2000.0), 33705.0);
    }

    #[test]
    fn test_get_distance() {
        assert_eq!(get_distance(10.0, 3.0, 5.0), 116112.01488218177);
    }

    #[test]
    fn test_get_charge1() {
        assert_eq!(get_charge1(10.0, 5.0, 2000.0), 0.0008900756564307966);
    }
}
