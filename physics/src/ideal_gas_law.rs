// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! The ideal gas law, also called the general gas equation, is the
//! equation of state of a hypothetical ideal gas.
//!
//! [Ideal gas law](https://en.wikipedia.org/wiki/Ideal_gas_law)

const UNIVERSAL_GAS_CONSTANT: f64 = 8.314_462;

#[must_use]
pub fn pressure_of_gas_system(moles: f64, kelvin: f64, volume: f64) -> f64 {
    debug_assert!(moles > 0.0);
    debug_assert!(kelvin > 0.0);
    debug_assert!(volume > 0.0);
    moles * kelvin * UNIVERSAL_GAS_CONSTANT / volume
}

#[must_use]
pub fn volume_of_gas_system(moles: f64, kelvin: f64, pressure: f64) -> f64 {
    debug_assert!(moles > 0.0);
    debug_assert!(kelvin > 0.0);
    debug_assert!(pressure > 0.0);
    moles * kelvin * UNIVERSAL_GAS_CONSTANT / pressure
}

#[cfg(test)]
mod tests {
    use super::{pressure_of_gas_system, volume_of_gas_system};

    #[test]
    fn test_pressure_of_gas_system() {
        assert!((pressure_of_gas_system(2.0, 100.0, 5.0) - 332.57848).abs() < f64::EPSILON);
        assert!((pressure_of_gas_system(0.5, 273.0, 0.004) - 283_731.015_75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_volume_of_gas_system() {
        assert!((volume_of_gas_system(2.0, 100.0, 5.0) - 332.57848).abs() < f64::EPSILON);
        assert!((volume_of_gas_system(0.5, 273.0, 0.004) - 283_731.015_75).abs() < f64::EPSILON);
    }
}
