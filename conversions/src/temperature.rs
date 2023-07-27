// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Convert between different units of temperature
//!
//! # References:
//! - [Celsius](https://en.wikipedia.org/wiki/Celsius)
//! - [Fahrenheit](https://en.wikipedia.org/wiki/Fahrenheit)
//! - [Kelvin](https://en.wikipedia.org/wiki/Kelvin)
//! - [Rankine scale](https://en.wikipedia.org/wiki/Rankine_scale)
//! - [Reaumur](https://en.wikipedia.org/wiki/R%C3%A9aumur_scale)

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
    Reaumur,
}

#[must_use]
#[allow(clippy::match_same_arms)]
pub fn temerature_conversion(value: f64, from_type: Unit, to_type: Unit) -> f64 {
    match (from_type, to_type) {
        // Celsius
        (Unit::Celsius, Unit::Celsius) => value,
        (Unit::Celsius, Unit::Fahrenheit) => value * 9.0 / 5.0 + 32.0,
        (Unit::Celsius, Unit::Kelvin) => value + 273.15,
        (Unit::Celsius, Unit::Rankine) => value * 9.0 / 5.0 + 491.67,
        (Unit::Celsius, Unit::Reaumur) => value / 1.25,

        // Fahrenheit
        (Unit::Fahrenheit, Unit::Celsius) => (value - 32.0) * 5.0 / 9.0,
        (Unit::Fahrenheit, Unit::Fahrenheit) => value,
        (Unit::Fahrenheit, Unit::Kelvin) => (value - 32.0) * 5.0 / 9.0 + 273.15,
        (Unit::Fahrenheit, Unit::Rankine) => value + 459.67,
        (Unit::Fahrenheit, Unit::Reaumur) => (value - 32.0) / 2.25,

        // Kelvin
        (Unit::Kelvin, Unit::Celsius) => value - 273.15,
        (Unit::Kelvin, Unit::Fahrenheit) => (value - 273.15) * 9.0 / 5.0 + 32.0,
        (Unit::Kelvin, Unit::Kelvin) => value,
        (Unit::Kelvin, Unit::Rankine) => value * 9.0 / 5.0,
        (Unit::Kelvin, Unit::Reaumur) => (value - 273.15) / 1.25,

        // Rankine
        (Unit::Rankine, Unit::Celsius) => (value - 491.67) * 5.0 / 9.0,
        (Unit::Rankine, Unit::Fahrenheit) => value - 459.67,
        (Unit::Rankine, Unit::Kelvin) => value * 5.0 / 9.0,
        (Unit::Rankine, Unit::Rankine) => value,
        (Unit::Rankine, Unit::Reaumur) => (value - 32.0 - 459.67) / 2.25,

        // Reaumur
        (Unit::Reaumur, Unit::Celsius) => value * 1.25,
        (Unit::Reaumur, Unit::Fahrenheit) => value.mul_add(2.25, 32.0),
        (Unit::Reaumur, Unit::Kelvin) => value.mul_add(1.25, 273.15),
        (Unit::Reaumur, Unit::Rankine) => value.mul_add(2.25, 32.0 + 459.67),
        (Unit::Reaumur, Unit::Reaumur) => value,
    }
}

#[cfg(test)]
mod tests {
    use super::{temerature_conversion, Unit};

    #[test]
    fn test_temperature_conversion() {
        const PAIRS: &[(Unit, Unit, f64, f64)] = &[
            (Unit::Celsius, Unit::Fahrenheit, 273.354, 524.037),
            (Unit::Celsius, Unit::Fahrenheit, 0.0, 32.0),
            (Unit::Celsius, Unit::Kelvin, 273.354, 546.504),
            (Unit::Celsius, Unit::Kelvin, 40.0, 313.15),
            (Unit::Celsius, Unit::Rankine, 273.354, 983.707),
            (Unit::Celsius, Unit::Rankine, 40.0, 563.67),
            (Unit::Fahrenheit, Unit::Celsius, 273.354, 134.086),
            (Unit::Fahrenheit, Unit::Celsius, 100.0, 37.778),
            (Unit::Fahrenheit, Unit::Kelvin, 273.354, 407.236),
            (Unit::Fahrenheit, Unit::Kelvin, 100.0, 310.928),
            (Unit::Fahrenheit, Unit::Rankine, 273.354, 733.024),
            (Unit::Fahrenheit, Unit::Rankine, 100.0, 559.67),
            (Unit::Kelvin, Unit::Celsius, 273.354, 0.204),
            (Unit::Kelvin, Unit::Celsius, 315.5, 42.35),
            (Unit::Kelvin, Unit::Fahrenheit, 273.354, 32.367),
            (Unit::Kelvin, Unit::Fahrenheit, 315.5, 108.23),
            (Unit::Kelvin, Unit::Rankine, 273.354, 492.037),
            (Unit::Kelvin, Unit::Rankine, 40.0, 72.0),
            (Unit::Rankine, Unit::Celsius, 273.354, -121.287),
            (Unit::Rankine, Unit::Celsius, 315.5, -97.872),
            (Unit::Rankine, Unit::Fahrenheit, 273.15, -186.52),
            (Unit::Rankine, Unit::Fahrenheit, 315.5, -144.17),
            (Unit::Rankine, Unit::Kelvin, 20.0, 11.111),
            (Unit::Rankine, Unit::Kelvin, 40.0, 22.222),
            (Unit::Reaumur, Unit::Kelvin, 0.0, 273.15),
            (Unit::Reaumur, Unit::Kelvin, 40.0, 323.15),
            (Unit::Reaumur, Unit::Fahrenheit, 0.0, 32.0),
            (Unit::Reaumur, Unit::Fahrenheit, 40.0, 122.0),
            (Unit::Reaumur, Unit::Celsius, 0.0, 0.0),
            (Unit::Reaumur, Unit::Celsius, 40.0, 50.0),
            (Unit::Reaumur, Unit::Rankine, 0.0, 491.67),
            (Unit::Reaumur, Unit::Rankine, 40.0, 581.67),
        ];
        for (from_type, to_type, val, to_val) in PAIRS {
            let val = temerature_conversion(*val, *from_type, *to_type);
            let val = (val * 1000.0).round() / 1000.0;
            let diff = (val - *to_val).abs();
            assert!(diff < f64::EPSILON);
        }
    }
}
