// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Conversion of weight units.
//!
//! Available units:
//! - kilogram
//! - gram
//! - milligram
//! - tonne
//! - long ton
//! - short ton
//! - pound
//! - stone
//! - ounce
//! - karrat
//! - atomic mass unit
//!
//! ## References
//! - [Kilogram](https://en.wikipedia.org/wiki/Kilogram)
//! - [Gram](https://en.wikipedia.org/wiki/Gram)
//! - [Tonne](https://en.wikipedia.org/wiki/Tonne)
//! - [Long Ton](https://en.wikipedia.org/wiki/Long_ton)
//! - [Short Ton](https://en.wikipedia.org/wiki/Short_ton)
//! - [Pound](https://en.wikipedia.org/wiki/Pound)
//! - [Ounce](https://en.wikipedia.org/wiki/Ounce)
//! - [Karat](https://en.wikipedia.org/wiki/Fineness#Karat)
//! - [Dalton](https://en.wikipedia.org/wiki/Dalton_(unit))
//! - [Stone](https://en.wikipedia.org/wiki/Stone_(unit))

#![allow(clippy::module_name_repetitions)]

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Unit {
    Kilogram,
    Gram,
    Milligram,
    Tonne,
    LongTon,
    ShortTon,
    Pound,
    Stone,
    Ounce,
    Karrat,
    AtomicMassUnit,
}

const MAP: &[(Unit, f64, f64)] = &[
    (Unit::Kilogram, 1.0, 1.0),
    (Unit::Gram, 1.0e3, 1.0e-3),
    (Unit::Milligram, 1.0e6, 1.0e-6),
    (Unit::Tonne, 1.0e-3, 1.0e3),
    (Unit::LongTon, 0.000_984_207_3, 1_016.046_08),
    (Unit::ShortTon, 0.001_102_312_2, 907.184),
    (Unit::Pound, 2.204_624_420_2, 0.453_592),
    (Unit::Stone, 0.157_473_172_8, 6.350_29),
    (Unit::Ounce, 35.273_990_723, 0.028_349_5),
    (Unit::Karrat, 5000.0, 0.0002),
    (Unit::AtomicMassUnit, 6.022_136_652e26, 1.660_540_199e-27),
];

/// Conversion of weight unit
#[must_use]
pub fn weight_conversion(value: f64, from_type: Unit, to_type: Unit) -> f64 {
    let from_tuple = MAP[from_type as usize];
    debug_assert!(from_tuple.0 == from_type);
    let to_tuple = MAP[to_type as usize];
    debug_assert!(to_tuple.0 == to_type);
    value * to_tuple.1 * from_tuple.2
}

#[cfg(test)]
mod tests {
    use super::{weight_conversion, Unit};

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_weight_conversion() {
        const PAIRS: &[(Unit, Unit, f64, f64)] = &[
            (Unit::Kilogram, Unit::Kilogram, 4.0, 4.0),
            (Unit::Kilogram, Unit::Gram, 1.0, 1000.0),
            (Unit::Kilogram, Unit::Milligram, 4.0, 4_000_000.0),
            (Unit::Kilogram, Unit::Tonne, 4.0, 0.004),
            (Unit::Kilogram, Unit::LongTon, 3.0, 0.002_952_621_9),
            (Unit::Kilogram, Unit::ShortTon, 1.0, 0.001_102_312_2),
            (Unit::Kilogram, Unit::Pound, 4.0, 8.818_497_680_8),
            (Unit::Kilogram, Unit::Stone, 5.0, 0.787_365_864_000_000_1),
            (Unit::Kilogram, Unit::Ounce, 4.0, 141.095_962_892),
            (Unit::Kilogram, Unit::Karrat, 3.0, 15000.0),
            (Unit::Kilogram, Unit::AtomicMassUnit, 1.0, 6.022_136_652e+26),
            (Unit::Gram, Unit::Kilogram, 1.0, 0.001),
            (Unit::Gram, Unit::Gram, 3.0, 3.0),
            (Unit::Gram, Unit::Milligram, 2.0, 2000.0),
            (Unit::Gram, Unit::Tonne, 4.0, 4e-06),
            (Unit::Gram, Unit::LongTon, 3.0, 2.952_621_9e-06),
            (Unit::Gram, Unit::ShortTon, 3.0, 3.306_936_600_000_000_3e-6),
            (Unit::Gram, Unit::Pound, 3.0, 0.006_613_873_260_6),
            (Unit::Gram, Unit::Stone, 4.0, 0.000_629_892_691_200_000_1),
            (Unit::Gram, Unit::Ounce, 1.0, 0.035_273_990_723),
            (Unit::Gram, Unit::Karrat, 2.0, 10.0),
            (Unit::Gram, Unit::AtomicMassUnit, 1.0, 6.022_136_652e+23),
            (Unit::Milligram, Unit::Kilogram, 1.0, 1e-06),
            (Unit::Milligram, Unit::Gram, 2.0, 0.002),
            (Unit::Milligram, Unit::Milligram, 3.0, 3.0),
            (Unit::Milligram, Unit::Tonne, 3.0, 3e-09),
            (Unit::Milligram, Unit::LongTon, 3.0, 2.952_621_9e-09),
            (Unit::Milligram, Unit::ShortTon, 1.0, 1.102_312_2e-09),
            (
                Unit::Milligram,
                Unit::Pound,
                3.0,
                6.613_873_260_599_999_5e-6,
            ),
            (Unit::Milligram, Unit::Ounce, 2.0, 7.054_798_144_599_999e-5),
            (Unit::Milligram, Unit::Karrat, 1.0, 0.005),
            (
                Unit::Milligram,
                Unit::AtomicMassUnit,
                1.0,
                6.022_136_652e+20,
            ),
            (Unit::Tonne, Unit::Kilogram, 2.0, 2000.0),
            (Unit::Tonne, Unit::Gram, 2.0, 2_000_000.0),
            (Unit::Tonne, Unit::Milligram, 3.0, 3_000_000_000.0),
            (Unit::Tonne, Unit::Tonne, 2.0, 2.0),
            (Unit::Tonne, Unit::LongTon, 3.0, 2.952_621_9),
            (Unit::Tonne, Unit::ShortTon, 2.0, 2.204_624_4),
            (Unit::Tonne, Unit::Pound, 3.0, 6_613.873_260_6),
            (Unit::Tonne, Unit::Ounce, 4.0, 141_095.962_891_999_98),
            (Unit::Tonne, Unit::Karrat, 4.0, 20_000_000.0),
            (Unit::Tonne, Unit::AtomicMassUnit, 1.0, 6.022_136_652e+29),
            (Unit::LongTon, Unit::Kilogram, 4.0, 4064.18432),
            (Unit::LongTon, Unit::Gram, 4.0, 4_064_184.32),
            (Unit::LongTon, Unit::Milligram, 3.0, 3_048_138_240.0),
            (Unit::LongTon, Unit::Tonne, 4.0, 4.064_184_32),
            (Unit::LongTon, Unit::LongTon, 3.0, 2.999_999_907_217_152),
            (Unit::LongTon, Unit::ShortTon, 1.0, 1.119_999_989_746_176),
            (Unit::LongTon, Unit::Pound, 3.0, 6_720.000_000_049_448),
            (Unit::LongTon, Unit::Ounce, 1.0, 35_840.000_000_060_514),
            (Unit::LongTon, Unit::Karrat, 4.0, 20_320_921.599_999_998),
            (
                Unit::LongTon,
                Unit::AtomicMassUnit,
                4.0,
                2.447_507_335_395_569_7e30,
            ),
            (Unit::ShortTon, Unit::Kilogram, 3.0, 2_721.551_999_999_999_7),
            (Unit::ShortTon, Unit::Gram, 3.0, 2_721_552.0),
            (Unit::ShortTon, Unit::Milligram, 1.0, 907_184_000.0),
            (Unit::ShortTon, Unit::Tonne, 4.0, 3.628_736),
            (Unit::ShortTon, Unit::LongTon, 3.0, 2.678_571_345_729_6),
            (Unit::ShortTon, Unit::ShortTon, 3.0, 2.999_999_972_534_4),
            (Unit::ShortTon, Unit::Pound, 2.0, 4_000.000_000_029_433_5),
            (Unit::ShortTon, Unit::Ounce, 4.0, 128_000.000_000_216_11),
            (Unit::ShortTon, Unit::Karrat, 4.0, 18_143_680.0),
            (
                Unit::ShortTon,
                Unit::AtomicMassUnit,
                1.0,
                5.463_186_016_507_968e29,
            ),
            (Unit::Pound, Unit::Kilogram, 4.0, 1.814_368),
            (Unit::Pound, Unit::Gram, 2.0, 907.184),
            (Unit::Pound, Unit::Milligram, 3.0, 1_360_776.0),
            (Unit::Pound, Unit::Tonne, 3.0, 0.001_360_776),
            (Unit::Pound, Unit::LongTon, 2.0, 0.000_892_857_115_243_2),
            (Unit::Pound, Unit::ShortTon, 1.0, 0.000_499_999_995_422_4),
            (Unit::Pound, Unit::Pound, 3.0, 3.000_000_000_022_075_2),
            (Unit::Pound, Unit::Ounce, 1.0, 16.000_000_000_027_015),
            (Unit::Pound, Unit::Karrat, 1.0, 2267.96),
            (
                Unit::Pound,
                Unit::AtomicMassUnit,
                4.0,
                1.092_637_203_301_593_6e27,
            ),
            (Unit::Stone, Unit::Kilogram, 5.0, 31.751_450_000_000_002),
            (Unit::Stone, Unit::Gram, 2.0, 12700.58),
            (Unit::Stone, Unit::Milligram, 3.0, 19_050_870.0),
            (Unit::Stone, Unit::Tonne, 3.0, 0.019_050_870),
            (Unit::Stone, Unit::LongTon, 3.0, 0.018_750_005_325_351_003),
            (Unit::Stone, Unit::ShortTon, 3.0, 0.021_000_006_421_614_002),
            (Unit::Stone, Unit::Pound, 2.0, 28.000_008_818_703_72),
            (Unit::Stone, Unit::Ounce, 1.0, 224.000_070_548_359_67),
            (Unit::Stone, Unit::Karrat, 2.0, 63502.9),
            (Unit::Ounce, Unit::Kilogram, 3.0, 0.085_048_5),
            (Unit::Ounce, Unit::Gram, 3.0, 85.0485),
            (Unit::Ounce, Unit::Milligram, 4.0, 113_398.0),
            (Unit::Ounce, Unit::Tonne, 4.0, 0.000_113_398),
            (Unit::Ounce, Unit::LongTon, 4.0, 0.000_111_607_139_405_4),
            (Unit::Ounce, Unit::ShortTon, 4.0, 0.000_124_999_998_855_6),
            (Unit::Ounce, Unit::Pound, 1.0, 0.062_500_000_000_459_9),
            (Unit::Ounce, Unit::Ounce, 2.0, 2.000_000_000_003_377),
            (Unit::Ounce, Unit::Karrat, 1.0, 141.7475),
            (
                Unit::Ounce,
                Unit::AtomicMassUnit,
                1.0,
                1.707_245_630_158_74e+25,
            ),
            (Unit::Karrat, Unit::Kilogram, 1.0, 0.0002),
            (Unit::Karrat, Unit::Gram, 4.0, 0.8),
            (Unit::Karrat, Unit::Milligram, 2.0, 400.0),
            (Unit::Karrat, Unit::Tonne, 2.0, 4.000_000_000_000_000_3e-7),
            (Unit::Karrat, Unit::LongTon, 3.0, 5.905_243_8e-07),
            (Unit::Karrat, Unit::ShortTon, 4.0, 8.818_497_600_000_002e-7),
            (Unit::Karrat, Unit::Pound, 1.0, 0.000_440_924_884_040_000_04),
            (Unit::Karrat, Unit::Ounce, 2.0, 0.014_109_596_289_2),
            (Unit::Karrat, Unit::Karrat, 4.0, 4.0),
            (Unit::Karrat, Unit::AtomicMassUnit, 4.0, 4.817_709_321_6e+23),
            (Unit::AtomicMassUnit, Unit::Kilogram, 4.0, 6.642_160_796e-27),
            (Unit::AtomicMassUnit, Unit::Gram, 2.0, 3.321_080_398e-24),
            (
                Unit::AtomicMassUnit,
                Unit::Milligram,
                2.0,
                3.321_080_398_000_000_2e-21,
            ),
            (
                Unit::AtomicMassUnit,
                Unit::Tonne,
                3.0,
                4.981_620_597_000_000_4e-30,
            ),
            (
                Unit::AtomicMassUnit,
                Unit::LongTon,
                3.0,
                4.902_947_357_397_758_4e-30,
            ),
            (
                Unit::AtomicMassUnit,
                Unit::ShortTon,
                1.0,
                1.830_433_719_948_128e-30,
            ),
            (
                Unit::AtomicMassUnit,
                Unit::Pound,
                3.0,
                1.098_260_242_031_750_4e-26,
            ),
            (
                Unit::AtomicMassUnit,
                Unit::Ounce,
                2.0,
                1.171_477_591_493_891_5e-25,
            ),
            (Unit::AtomicMassUnit, Unit::Karrat, 2.0, 1.660_540_199e-23),
            (
                Unit::AtomicMassUnit,
                Unit::AtomicMassUnit,
                2.0,
                1.999_999_998_903_455,
            ),
        ];
        for (from_type, to_type, val, to_val) in PAIRS {
            assert!((weight_conversion(*val, *from_type, *to_type) - to_val).abs() < f64::EPSILON);
        }
    }
}
