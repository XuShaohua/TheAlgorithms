// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Conversion of volume units.
//!
//! Available Units:
//! - [Cubic_metre](https://en.wikipedia.org/wiki/Cubic_metre)
//! - [Litre](https://en.wikipedia.org/wiki/Litre)
//! - [KiloLitre](https://en.wiktionary.org/wiki/kilolitre)
//! - [Gallon](https://en.wikipedia.org/wiki/Gallon)
//! - [Cubic_yard](https://en.wikipedia.org/wiki/Cubic_yard)
//! - [Cubic_foot](https://en.wikipedia.org/wiki/Cubic_foot)
//! - [Cup](https://en.wikipedia.org/wiki/Cup_(unit))

#![allow(clippy::module_name_repetitions)]

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Unit {
    CubicMeter,
    Litre,
    KiloLitre,
    Gallon,
    CubicYard,
    CubicFoot,
    Cup,
}

pub const METRIC_CONVERSION: &[(Unit, f64, f64)] = &[
    (Unit::CubicMeter, 1.0, 1.0),
    (Unit::Litre, 0.001, 1000.0),
    (Unit::KiloLitre, 1.0, 1.0),
    (Unit::Gallon, 0.00454, 264.172),
    (Unit::CubicYard, 0.76455, 1.30795),
    (Unit::CubicFoot, 0.028, 35.3147),
    (Unit::Cup, 0.000_236_588, 4226.75),
];

/// Conversion between volume units.
#[must_use]
pub fn volume_conversion(value: f64, from_type: Unit, to_type: Unit) -> f64 {
    let from_tuple = METRIC_CONVERSION[from_type as usize];
    let to_tuple = METRIC_CONVERSION[to_type as usize];
    value * from_tuple.1 * to_tuple.2
}

#[cfg(test)]
mod tests {
    use super::{volume_conversion, Unit};

    #[test]
    fn test_volume_conversion() {
        const PAIRS: &[(f64, Unit, Unit, f64)] = &[
            (4.0, Unit::CubicMeter, Unit::Litre, 4000.0),
            (1.0, Unit::Litre, Unit::Gallon, 0.264_172),
            (1.0, Unit::KiloLitre, Unit::CubicMeter, 1.0),
            (3.0, Unit::Gallon, Unit::CubicYard, 0.017_814_279),
            (2.0, Unit::CubicYard, Unit::Litre, 1529.1),
            (4.0, Unit::CubicFoot, Unit::Cup, 473.396),
            (1.0, Unit::Cup, Unit::KiloLitre, 0.000_236_588),
        ];
        for (value, from_type, to_type, to_value) in PAIRS {
            assert_eq!(volume_conversion(*value, *from_type, *to_type), *to_value);
        }
    }
}
