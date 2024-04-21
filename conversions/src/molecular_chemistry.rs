// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Functions useful for doing molecular chemistry:
//! - `molarity_to_normality`
//! - `moles_to_pressure`
//! - `moles_to_volume`
//! - `pressure_and_volume_to_temperature`

/// Convert molarity to normality.
///
/// Volume is taken in litres.
///
/// References:
/// - [Equivalent_concentration](https://en.wikipedia.org/wiki/Equivalent_concentration)
/// - [Molar_concentration](https://en.wikipedia.org/wiki/Molar_concentration)
#[must_use]
pub fn molarity_to_normality(nfactor: i32, moles: f64, volume: f64) -> f64 {
    (moles / volume * f64::from(nfactor)).round()
}

/// Convert moles to pressure.
///
/// Ideal gas laws are used. Temperature is taken in kelvin. Volume is taken in litres.
/// Pressure has atm as SI unit.
///
/// References:
/// - [Gas_laws](https://en.wikipedia.org/wiki/Gas_laws)
/// - [Pressure](https://en.wikipedia.org/wiki/Pressure)
/// - [Temperature](https://en.wikipedia.org/wiki/Temperature)
#[must_use]
pub fn moles_to_pressure(volume: f64, moles: f64, temperature: f64) -> f64 {
    (moles * 0.0821 * temperature / volume).round()
}

/// Convert moles to volume.
///
/// Ideal gas laws are used. Temperature is taken in kelvin. Volume is taken in litres.
/// Pressure has atm as SI unit.
///
/// References:
/// - [Gas_laws](https://en.wikipedia.org/wiki/Gas_laws)
/// - [Pressure](https://en.wikipedia.org/wiki/Pressure)
/// - [Temperature](https://en.wikipedia.org/wiki/Temperature)
#[must_use]
pub fn moles_to_volume(pressure: f64, moles: f64, temperature: f64) -> f64 {
    (moles * 0.0821 * temperature / pressure).round()
}

/// Convert pressure and volume to temperature.
///
/// Ideal gas laws are used. Temperature is taken in kelvin.
/// Volume is taken in litres. Pressure has atm as SI unit.
///
/// References:
/// - [Gas_laws](https://en.wikipedia.org/wiki/Gas_laws)
/// - [Pressure](https://en.wikipedia.org/wiki/Pressure)
/// - [Temperature](https://en.wikipedia.org/wiki/Temperature)
#[must_use]
pub fn pressure_and_volume_to_temperature(pressure: f64, moles: f64, volume: f64) -> f64 {
    (pressure * volume / (0.0821 * moles)).round()
}

#[cfg(test)]
mod tests {
    use super::{
        molarity_to_normality, moles_to_pressure, moles_to_volume,
        pressure_and_volume_to_temperature,
    };

    #[test]
    fn test_molarity_to_normality() {
        assert!((molarity_to_normality(2, 3.1, 0.31) - 20.0).abs() < f64::EPSILON);
        assert!((molarity_to_normality(4, 11.4, 5.7) - 8.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_moles_to_pressure() {
        assert!((moles_to_pressure(0.82, 3.0, 300.0) - 90.0).abs() < f64::EPSILON);
        assert!((moles_to_pressure(8.2, 5.0, 200.0) - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_moles_to_volume() {
        assert!((moles_to_volume(0.82, 3.0, 300.0) - 90.0).abs() < f64::EPSILON);
        assert!((moles_to_volume(8.2, 5.0, 200.0) - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_pressure_and_volume_to_temperature() {
        assert!((pressure_and_volume_to_temperature(0.82, 1.0, 2.0) - 20.0).abs() < f64::EPSILON);
        assert!((pressure_and_volume_to_temperature(8.2, 5.0, 3.0) - 60.0).abs() < f64::EPSILON);
    }
}
