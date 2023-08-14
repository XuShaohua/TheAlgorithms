// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! [Inductive reactance](https://en.wikipedia.org/wiki/Electrical_reactance#Inductive_reactance)

use std::f64::consts::PI;

/// inductance with units in Henries
/// frequency with units in Hertz
/// reactance with units in Ohms
#[must_use]
pub fn get_inductance(frequency: f64, reactance: f64) -> f64 {
    debug_assert!(frequency > 0.0);
    debug_assert!(reactance > 0.0);
    reactance / (2.0 * PI * frequency)
}

#[must_use]
pub fn get_frequency(inductance: f64, reactance: f64) -> f64 {
    debug_assert!(inductance > 0.0);
    debug_assert!(reactance > 0.0);
    reactance / (2.0 * PI * inductance)
}

#[must_use]
pub fn get_reactance(inductance: f64, frequency: f64) -> f64 {
    debug_assert!(inductance > 0.0);
    debug_assert!(frequency > 0.0);
    2.0 * PI * frequency * inductance
}

#[cfg(test)]
mod tests {
    use super::{get_frequency, get_inductance, get_reactance};
    #[test]
    fn test_get_inductance() {
        assert_eq!(get_inductance(10e3, 50.0), 0.0007957747154594767);
    }

    #[test]
    fn test_get_frequency() {
        assert_eq!(get_frequency(35e-3, 50.0), 227.36420441699332);
    }

    #[test]
    fn test_get_reactance() {
        assert_eq!(get_reactance(35e-6, 1e3), 0.2199114857512855);
    }
}
