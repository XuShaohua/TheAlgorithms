// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Electrical impedance is the measure of the opposition that a
//! circuit presents to a current when a voltage is applied.
//!
//! Impedance extends the concept of resistance to alternating current (AC) circuits.
//! [Electrical impedance](https://en.wikipedia.org/wiki/Electrical_impedance)

#[must_use]
pub fn get_resistance(reactance: f64, impedance: f64) -> f64 {
    impedance.mul_add(impedance, -reactance.powi(2)).sqrt()
}

#[must_use]
pub fn get_reactance(resistance: f64, impedance: f64) -> f64 {
    impedance.mul_add(impedance, -resistance.powi(2)).sqrt()
}

#[must_use]
pub fn get_impedance(resistance: f64, reactance: f64) -> f64 {
    resistance.hypot(reactance)
}

#[cfg(test)]
mod tests {
    use super::{get_impedance, get_reactance, get_resistance};

    #[test]
    fn test_get_resistance() {
        assert!((get_resistance(4.0, 5.0) - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_get_reactance() {
        assert!((get_reactance(4.0, 5.0) - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_get_impedance() {
        assert!((get_impedance(3.0, 4.0) - 5.0).abs() < f64::EPSILON);
    }
}
