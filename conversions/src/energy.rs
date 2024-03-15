// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Conversion of energy units.
//!
//! Available units:
//! - joule
//! - kilojoule
//! - megajoule
//! - gigajoule,
//! - wattsecond
//! - watthour
//! - kilowatthour
//! - newtonmeter
//! - `calorie_nutr`,
//! - `kilocalorie_nutr`
//! - electronvolt
//! - `britishthermalunit_it`
//! - footpound
//!
//! ## References
//! - [Units of Energy](https://en.wikipedia.org/wiki/Units_of_energy)
//! - [Joule](https://en.wikipedia.org/wiki/Joule)
//! - [Kilowatt Hour](https://en.wikipedia.org/wiki/Kilowatt-hour)
//! - [Newton Meter](https://en.wikipedia.org/wiki/Newton-metre)
//! - [Calorie](https://en.wikipedia.org/wiki/Calorie)
//! - [Electronvolt](https://en.wikipedia.org/wiki/Electronvolt)
//! - [British thermal unit](https://en.wikipedia.org/wiki/British_thermal_unit)
//! - [Foot pound](https://en.wikipedia.org/wiki/Foot-pound_(energy))
//! - [Energy Converter](https://www.unitconverters.net/energy-converter.html)

#![allow(clippy::module_name_repetitions)]

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Unit {
    Joule,
    KiloJoule,
    MegaJoule,
    GigaJoule,
    WattSecond,
    WattHour,
    KiloWattHour,
    NewtonMeter,
    CalorieNutr,
    KiloCalorieNutr,
    ElectronVolt,
    BritishThermalUnitIt,
    FootPound,
}

pub const MAP: &[(Unit, f64)] = &[
    (Unit::Joule, 1.0),
    (Unit::KiloJoule, 1000.0),
    (Unit::MegaJoule, 1_000_000.0),
    (Unit::GigaJoule, 1_000_000_000.0),
    (Unit::WattSecond, 1.0),
    (Unit::WattHour, 3_600.0),
    (Unit::KiloWattHour, 3_600_000.0),
    (Unit::NewtonMeter, 1.0),
    (Unit::CalorieNutr, 4_186.8),
    (Unit::KiloCalorieNutr, 4_186_800.0),
    (Unit::ElectronVolt, 1.602_176_634e-19),
    (Unit::BritishThermalUnitIt, 1_055.055_85),
    (Unit::FootPound, 1.355_818),
];

/// Conversion of energy units.
#[must_use]
pub fn energy_conversion(value: f64, from_type: Unit, to_type: Unit) -> f64 {
    let from_tuple = MAP[from_type as usize];
    debug_assert!(from_tuple.0 == from_type);
    let to_tuple = MAP[to_type as usize];
    debug_assert!(to_tuple.0 == to_type);
    value * from_tuple.1 / to_tuple.1
}

#[cfg(test)]
mod tests {
    use super::{energy_conversion, Unit};

    #[test]
    fn test_energy_conversion() {
        const PAIRS: &[(f64, Unit, Unit, f64)] = &[
            (1.0, Unit::Joule, Unit::Joule, 1.0),
            (1.0, Unit::Joule, Unit::KiloJoule, 0.001),
            (1.0, Unit::Joule, Unit::MegaJoule, 1e-06),
            (1.0, Unit::Joule, Unit::GigaJoule, 1e-09),
            (1.0, Unit::Joule, Unit::WattSecond, 1.0),
            (
                1.0,
                Unit::Joule,
                Unit::WattHour,
                0.000_277_777_777_777_777_8,
            ),
            (
                1.0,
                Unit::Joule,
                Unit::KiloWattHour,
                2.777_777_777_777_777_6e-7,
            ),
            (1.0, Unit::Joule, Unit::NewtonMeter, 1.0),
            (
                1.0,
                Unit::Joule,
                Unit::CalorieNutr,
                0.000_238_845_896_627_495_92,
            ),
            (
                1.0,
                Unit::Joule,
                Unit::KiloCalorieNutr,
                2.388_458_966_274_959e-7,
            ),
            (
                1.0,
                Unit::Joule,
                Unit::ElectronVolt,
                6.241_509_074_460_763e18,
            ),
            (
                1.0,
                Unit::Joule,
                Unit::BritishThermalUnitIt,
                0.000_947_817_122_667_013_4,
            ),
            (1.0, Unit::Joule, Unit::FootPound, 0.737_562_121_169_655_6),
            (1000.0, Unit::Joule, Unit::MegaJoule, 0.001),
            (1000.0, Unit::CalorieNutr, Unit::KiloCalorieNutr, 1.0),
            (10.0, Unit::KiloWattHour, Unit::Joule, 36_000_000.0),
            (
                1.0,
                Unit::BritishThermalUnitIt,
                Unit::FootPound,
                778.169_230_678_453_9,
            ),
        ];

        for (value, unit_from, unit_to, to_value) in PAIRS {
            assert_eq!(energy_conversion(*value, *unit_from, *unit_to), *to_value);
        }
    }
}
