// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Conversion of pressure units.
//!
//! Available Units:
//! - Pascal,
//! - Bar,
//! - Kilopascal,
//! - Megapascal,
//! - psi(pound per square inch),
//! - inHg(in mercury column),
//! - torr,
//! - atm
//!
//! ## References
//! - [Pascal](https://en.wikipedia.org/wiki/Pascal_(unit))
//! - [psi](https://en.wikipedia.org/wiki/Pound_per_square_inch)
//! - [inch of mercury](https://en.wikipedia.org/wiki/Inch_of_mercury)
//! - [Torr](https://en.wikipedia.org/wiki/Torr)
//! - [Standard atmosphre](https://en.wikipedia.org/wiki/Standard_atmosphere_(unit))
//! - [units of pressure](https://msestudent.com/what-are-the-units-of-pressure/)
//! - [pressure converter](https://www.unitconverters.net/pressure-converter.html)

#![allow(clippy::module_name_repetitions)]

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Unit {
    Atm,
    Pascal,
    Bar,
    KiloPascal,
    MegaPascal,
    Psi,
    InHg,
    Torr,
}

const MAP: &[(Unit, f64, f64)] = &[
    (Unit::Atm, 1.0, 1.0),
    (Unit::Pascal, 0.000_009_8, 101_325.0),
    (Unit::Bar, 0.986_923, 1.01325),
    (Unit::KiloPascal, 0.009_869_23, 101.325),
    (Unit::MegaPascal, 9.86923, 0.101_325),
    (Unit::Psi, 0.068_046, 14.6959),
    (Unit::InHg, 0.033_421_1, 29.9213),
    (Unit::Torr, 0.001_315_79, 760.0),
];

/// Conversion between pressure units.
#[must_use]
pub fn pressure_conversion(value: f64, from_type: Unit, to_type: Unit) -> f64 {
    let from_tuple = MAP[from_type as usize];
    debug_assert!(from_tuple.0 == from_type);
    let to_tuple = MAP[to_type as usize];
    debug_assert!(to_tuple.0 == to_type);
    value * from_tuple.1 * to_tuple.2
}

#[cfg(test)]
mod tests {
    use super::{pressure_conversion, Unit};

    #[test]
    fn test_pressure_conversion() {
        const PAIRS: &[(f64, Unit, Unit, f64)] = &[
            (4.0, Unit::Atm, Unit::Pascal, 405_300.0),
            (1.0, Unit::Pascal, Unit::Psi, 0.000_144_019_819_999_999_98),
            (1.0, Unit::Bar, Unit::Atm, 0.986_923),
            (3.0, Unit::KiloPascal, Unit::Bar, 0.029_999_991_892_499_998),
            (2.0, Unit::MegaPascal, Unit::Psi, 290.074_434_314),
            (4.0, Unit::Psi, Unit::Torr, 206.859_84),
            (1.0, Unit::InHg, Unit::Atm, 0.033_421_1),
            (1.0, Unit::Torr, Unit::Psi, 0.019_336_718_261_000_002),
        ];
        for (val, from_unit, to_unit, to_val) in PAIRS {
            assert_eq!(pressure_conversion(*val, *from_unit, *to_unit), *to_val);
        }
    }
}
