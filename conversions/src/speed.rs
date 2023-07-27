// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Convert speed units
//!
//! Supported units:
//! - [Kilometres_per_hour](https://en.wikipedia.org/wiki/Kilometres_per_hour)
//! - [Miles_per_hour](https://en.wikipedia.org/wiki/Miles_per_hour)
//! - [Knot](https://en.wikipedia.org/wiki/Knot_(unit))
//! - [Metre_per_second](https://en.wikipedia.org/wiki/Metre_per_second)

#![allow(clippy::module_name_repetitions)]

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Unit {
    KilometersPerHour,
    MeterPerSecond,
    MilesPerHour,
    Knot,
}

pub const CHART: &[(Unit, f64)] = &[
    (Unit::KilometersPerHour, 1.0),
    (Unit::MeterPerSecond, 3.6),
    (Unit::MilesPerHour, 1.609_344),
    (Unit::Knot, 1.852),
];

pub const CHART_INVERSE: &[(Unit, f64)] = &[
    (Unit::KilometersPerHour, 1.0),
    (Unit::MeterPerSecond, 0.277_777_778),
    (Unit::MilesPerHour, 0.621_371_192),
    (Unit::Knot, 0.539_956_803),
];

/// Convert speed from one unit to another.
#[must_use]
pub fn convert_speed(speed: f64, unit_from: Unit, unit_to: Unit) -> f64 {
    let from_tuple = CHART[unit_from as usize];
    let to_tuple = CHART_INVERSE[unit_to as usize];
    // Round with 3 digits.
    (speed * from_tuple.1 * to_tuple.1 * 1000.0).round() / 1000.0
}

#[cfg(test)]
mod tests {
    use super::{convert_speed, Unit};

    #[test]
    fn test_convert_speed() {
        const PAIRS: &[(f64, Unit, Unit, f64)] = &[
            (100.0, Unit::KilometersPerHour, Unit::MeterPerSecond, 27.778),
            (100.0, Unit::KilometersPerHour, Unit::MilesPerHour, 62.137),
            (100.0, Unit::KilometersPerHour, Unit::Knot, 53.996),
            (100.0, Unit::MeterPerSecond, Unit::KilometersPerHour, 360.0),
            (100.0, Unit::MeterPerSecond, Unit::MilesPerHour, 223.694),
            (100.0, Unit::MeterPerSecond, Unit::Knot, 194.384),
            (100.0, Unit::MilesPerHour, Unit::KilometersPerHour, 160.934),
            (100.0, Unit::MilesPerHour, Unit::MeterPerSecond, 44.704),
            (100.0, Unit::MilesPerHour, Unit::Knot, 86.898),
            (100.0, Unit::Knot, Unit::KilometersPerHour, 185.2),
            (100.0, Unit::Knot, Unit::MeterPerSecond, 51.444),
            (100.0, Unit::Knot, Unit::MilesPerHour, 115.078),
        ];

        for (value, unit_from, unit_to, value_to) in PAIRS {
            assert_eq!(convert_speed(*value, *unit_from, *unit_to), *value_to);
        }
    }
}
