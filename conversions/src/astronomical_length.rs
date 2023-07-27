// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Conversion of length units.
//!
//! Available Units:
//! - Meter
//! - Kilometer
//! - Megameter
//! - Gigameter
//! - Terameter
//! - Petameter
//! - Exameter
//! - Zettameter
//! - Yottameter
//!
//! ## References
//! - [Meter](https://en.wikipedia.org/wiki/Meter)
//! - [Kilometer](https://en.wikipedia.org/wiki/Kilometer)
//! - [Orders of magnitude](https://en.wikipedia.org/wiki/Orders_of_magnitude_(length))

#![allow(clippy::module_name_repetitions)]

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Unit {
    Meter,
    Kilometer,
    Megameter,
    Gigameter,
    Terameter,
    Petameter,
    Exameter,
    Zettameter,
    Yottameter,
}

/// Exponent of the factor(meter)
const EXPONENTS: &[(Unit, i32)] = &[
    (Unit::Meter, 0),
    (Unit::Kilometer, 3),
    (Unit::Megameter, 6),
    (Unit::Gigameter, 9),
    (Unit::Terameter, 12),
    (Unit::Petameter, 15),
    (Unit::Exameter, 18),
    (Unit::Zettameter, 21),
    (Unit::Yottameter, 24),
];

/// Conversion between astronomical length units.
#[must_use]
pub fn length_conversion(value: f64, from_type: Unit, to_type: Unit) -> f64 {
    let from_tuple = EXPONENTS[from_type as usize];
    debug_assert!(from_tuple.0 == from_type);
    let to_tuple = EXPONENTS[to_type as usize];
    debug_assert!(to_tuple.0 == to_type);

    let exponent = from_tuple.1 - to_tuple.1;
    value * 10.0_f64.powi(exponent)
}

#[cfg(test)]
mod tests {
    use super::{length_conversion, Unit};

    #[test]
    fn test_length_conversion() {
        const PAIRS: &[(f64, Unit, Unit, f64)] = &[
            (1.0, Unit::Meter, Unit::Kilometer, 0.001),
            (1.0, Unit::Meter, Unit::Megameter, 1e-06),
            (1.0, Unit::Gigameter, Unit::Meter, 1_000_000_000.0),
            (1.0, Unit::Gigameter, Unit::Terameter, 0.001),
            (1.0, Unit::Petameter, Unit::Terameter, 1000.0),
            (1.0, Unit::Petameter, Unit::Exameter, 0.001),
            (1.0, Unit::Terameter, Unit::Zettameter, 1e-09),
            (1.0, Unit::Yottameter, Unit::Zettameter, 1000.0),
        ];
        for (val, from_type, to_type, to_val) in PAIRS {
            assert_eq!(length_conversion(*val, *from_type, *to_type), *to_val);
        }
    }
}
