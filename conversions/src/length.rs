// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Conversion of length units.
//! Available Units:
//! - [Metre](https://en.wikipedia.org/wiki/Meter)
//! - [Kilometer](https://en.wikipedia.org/wiki/Kilometer)
//! - [Centimeter](https://en.wikipedia.org/wiki/Centimeter)
//! - [Millimeter](https://en.wikipedia.org/wiki/Millimeter)
//! - [Inch](https://en.wikipedia.org/wiki/Inch)
//! - [Yard](https://en.wikipedia.org/wiki/Yard)
//! - [Foot](https://en.wikipedia.org/wiki/Foot)
//! - [Mile](https://en.wikipedia.org/wiki/Mile)

#![allow(clippy::module_name_repetitions)]

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Unit {
    Meter,
    Kilometer,
    Centimeter,
    Millimeter,
    Inch,
    Yard,
    Foot,
    Mile,
}

const MAP: &[(Unit, f64, f64)] = &[
    (Unit::Meter, 1.0, 1.0),
    (Unit::Kilometer, 1000.0, 0.001),
    (Unit::Centimeter, 0.01, 100.0),
    (Unit::Millimeter, 0.001, 1000.0),
    (Unit::Inch, 0.0254, 39.3701),
    (Unit::Yard, 0.9144, 1.09361),
    (Unit::Foot, 0.3048, 3.28084),
    (Unit::Mile, 1609.34, 0.000_621_371),
];

/// Conversion between length units.
#[must_use]
pub fn length_conversion(value: f64, from_type: Unit, to_type: Unit) -> f64 {
    let from_tuple = MAP[from_type as usize];
    debug_assert!(from_tuple.0 == from_type);
    let to_tuple = MAP[to_type as usize];
    debug_assert!(to_tuple.0 == to_type);
    value * from_tuple.1 * to_tuple.2
}

#[cfg(test)]
mod tests {
    use super::{length_conversion, Unit};

    #[test]
    fn test_length_conversion() {
        const PAIRS: &[(f64, Unit, Unit, f64)] = &[
            (4.0, Unit::Meter, Unit::Foot, 13.12336),
            (4.0, Unit::Meter, Unit::Foot, 13.12336),
            (1.0, Unit::Meter, Unit::Kilometer, 0.001),
            (1.0, Unit::Kilometer, Unit::Inch, 39_370.1),
            (3.0, Unit::Kilometer, Unit::Mile, 1.8_641_130_000_000_001),
            (2.0, Unit::Foot, Unit::Meter, 0.6096),
            (4.0, Unit::Foot, Unit::Yard, 1.333_329_312),
            (1.0, Unit::Inch, Unit::Meter, 0.0254),
            (2.0, Unit::Inch, Unit::Mile, 3.15_656_468e-05),
            (2.0, Unit::Centimeter, Unit::Millimeter, 20.0),
            (2.0, Unit::Centimeter, Unit::Yard, 0.0_218_722),
            (4.0, Unit::Yard, Unit::Meter, 3.6576),
            (4.0, Unit::Yard, Unit::Kilometer, 0.0_036_576),
            (3.0, Unit::Foot, Unit::Meter, 0.9_144_000_000_000_001),
            (3.0, Unit::Foot, Unit::Inch, 36.00_001_944),
            (4.0, Unit::Mile, Unit::Kilometer, 6.43_736),
            (2.0, Unit::Mile, Unit::Inch, 126_719.753_468),
            (3.0, Unit::Millimeter, Unit::Centimeter, 0.3),
            (3.0, Unit::Millimeter, Unit::Inch, 0.1_181_103),
        ];

        for (val, from_unit, to_unit, to_val) in PAIRS {
            assert_eq!(length_conversion(*val, *from_unit, *to_unit), *to_val);
        }
    }
}
