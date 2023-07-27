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
    fn test_weight_conversion() {
        const PAIRS: &[(Unit, Unit, f64, f64)] = &[
            (Unit::Kilogram, Unit::Kilogram, 4.0, 4.0),
            (Unit::Kilogram, Unit::Gram, 1.0, 1000.0),
            (Unit::Kilogram, Unit::Milligram, 4.0, 4000000.0),
            (Unit::Kilogram, Unit::Tonne, 4.0, 0.004),
            (Unit::Kilogram, Unit::LongTon, 3.0, 0.0029526219),
            (Unit::Kilogram, Unit::ShortTon, 1.0, 0.0011023122),
            (Unit::Kilogram, Unit::Pound, 4.0, 8.8184976808),
            (Unit::Kilogram, Unit::Stone, 5.0, 0.7873658640000001),
            (Unit::Kilogram, Unit::Ounce, 4.0, 141.095962892),
            (Unit::Kilogram, Unit::Karrat, 3.0, 15000.0),
            (Unit::Kilogram, Unit::AtomicMassUnit, 1.0, 6.022136652e+26),
            (Unit::Gram, Unit::Kilogram, 1.0, 0.001),
            (Unit::Gram, Unit::Gram, 3.0, 3.0),
            (Unit::Gram, Unit::Milligram, 2.0, 2000.0),
            (Unit::Gram, Unit::Tonne, 4.0, 4e-06),
            (Unit::Gram, Unit::LongTon, 3.0, 2.9526219e-06),
            (Unit::Gram, Unit::ShortTon, 3.0, 3.3069366000000003e-06),
            (Unit::Gram, Unit::Pound, 3.0, 0.0066138732606),
            (Unit::Gram, Unit::Stone, 4.0, 0.0006298926912000001),
            (Unit::Gram, Unit::Ounce, 1.0, 0.035273990723),
            (Unit::Gram, Unit::Karrat, 2.0, 10.0),
            (Unit::Gram, Unit::AtomicMassUnit, 1.0, 6.022136652e+23),
            (Unit::Milligram, Unit::Kilogram, 1.0, 1e-06),
            (Unit::Milligram, Unit::Gram, 2.0, 0.002),
            (Unit::Milligram, Unit::Milligram, 3.0, 3.0),
            (Unit::Milligram, Unit::Tonne, 3.0, 3e-09),
            (Unit::Milligram, Unit::LongTon, 3.0, 2.9526219e-09),
            (Unit::Milligram, Unit::ShortTon, 1.0, 1.1023122e-09),
            (Unit::Milligram, Unit::Pound, 3.0, 6.6138732605999995e-06),
            (Unit::Milligram, Unit::Ounce, 2.0, 7.054798144599999e-05),
            (Unit::Milligram, Unit::Karrat, 1.0, 0.005),
            (Unit::Milligram, Unit::AtomicMassUnit, 1.0, 6.022136652e+20),
            (Unit::Tonne, Unit::Kilogram, 2.0, 2000.0),
            (Unit::Tonne, Unit::Gram, 2.0, 2000000.0),
            (Unit::Tonne, Unit::Milligram, 3.0, 3000000000.0),
            (Unit::Tonne, Unit::Tonne, 2.0, 2.0),
            (Unit::Tonne, Unit::LongTon, 3.0, 2.9526219),
            (Unit::Tonne, Unit::ShortTon, 2.0, 2.2046244),
            (Unit::Tonne, Unit::Pound, 3.0, 6613.8732606),
            (Unit::Tonne, Unit::Ounce, 4.0, 141095.96289199998),
            (Unit::Tonne, Unit::Karrat, 4.0, 20000000.0),
            (Unit::Tonne, Unit::AtomicMassUnit, 1.0, 6.022136652e+29),
            (Unit::LongTon, Unit::Kilogram, 4.0, 4064.18432),
            (Unit::LongTon, Unit::Gram, 4.0, 4064184.32),
            (Unit::LongTon, Unit::Milligram, 3.0, 3048138240.0),
            (Unit::LongTon, Unit::Tonne, 4.0, 4.06418432),
            (Unit::LongTon, Unit::LongTon, 3.0, 2.999999907217152),
            (Unit::LongTon, Unit::ShortTon, 1.0, 1.119999989746176),
            (Unit::LongTon, Unit::Pound, 3.0, 6720.000000049448),
            (Unit::LongTon, Unit::Ounce, 1.0, 35840.000000060514),
            (Unit::LongTon, Unit::Karrat, 4.0, 20320921.599999998),
            (
                Unit::LongTon,
                Unit::AtomicMassUnit,
                4.0,
                2.4475073353955697e+30,
            ),
            (Unit::ShortTon, Unit::Kilogram, 3.0, 2721.5519999999997),
            (Unit::ShortTon, Unit::Gram, 3.0, 2721552.0),
            (Unit::ShortTon, Unit::Milligram, 1.0, 907184000.0),
            (Unit::ShortTon, Unit::Tonne, 4.0, 3.628736),
            (Unit::ShortTon, Unit::LongTon, 3.0, 2.6785713457296),
            (Unit::ShortTon, Unit::ShortTon, 3.0, 2.9999999725344),
            (Unit::ShortTon, Unit::Pound, 2.0, 4000.0000000294335),
            (Unit::ShortTon, Unit::Ounce, 4.0, 128000.00000021611),
            (Unit::ShortTon, Unit::Karrat, 4.0, 18143680.0),
            (
                Unit::ShortTon,
                Unit::AtomicMassUnit,
                1.0,
                5.463186016507968e+29,
            ),
            (Unit::Pound, Unit::Kilogram, 4.0, 1.814368),
            (Unit::Pound, Unit::Gram, 2.0, 907.184),
            (Unit::Pound, Unit::Milligram, 3.0, 1360776.0),
            (Unit::Pound, Unit::Tonne, 3.0, 0.001360776),
            (Unit::Pound, Unit::LongTon, 2.0, 0.0008928571152432),
            (Unit::Pound, Unit::ShortTon, 1.0, 0.0004999999954224),
            (Unit::Pound, Unit::Pound, 3.0, 3.0000000000220752),
            (Unit::Pound, Unit::Ounce, 1.0, 16.000000000027015),
            (Unit::Pound, Unit::Karrat, 1.0, 2267.96),
            (
                Unit::Pound,
                Unit::AtomicMassUnit,
                4.0,
                1.0926372033015936e+27,
            ),
            (Unit::Stone, Unit::Kilogram, 5.0, 31.751450000000002),
            (Unit::Stone, Unit::Gram, 2.0, 12700.58),
            (Unit::Stone, Unit::Milligram, 3.0, 19050870.0),
            (Unit::Stone, Unit::Tonne, 3.0, 0.019050870),
            (Unit::Stone, Unit::LongTon, 3.0, 0.018750005325351003),
            (Unit::Stone, Unit::ShortTon, 3.0, 0.021000006421614002),
            (Unit::Stone, Unit::Pound, 2.0, 28.00000881870372),
            (Unit::Stone, Unit::Ounce, 1.0, 224.00007054835967),
            (Unit::Stone, Unit::Karrat, 2.0, 63502.9),
            (Unit::Ounce, Unit::Kilogram, 3.0, 0.0850485),
            (Unit::Ounce, Unit::Gram, 3.0, 85.0485),
            (Unit::Ounce, Unit::Milligram, 4.0, 113398.0),
            (Unit::Ounce, Unit::Tonne, 4.0, 0.000113398),
            (Unit::Ounce, Unit::LongTon, 4.0, 0.0001116071394054),
            (Unit::Ounce, Unit::ShortTon, 4.0, 0.0001249999988556),
            (Unit::Ounce, Unit::Pound, 1.0, 0.0625000000004599),
            (Unit::Ounce, Unit::Ounce, 2.0, 2.000000000003377),
            (Unit::Ounce, Unit::Karrat, 1.0, 141.7475),
            (Unit::Ounce, Unit::AtomicMassUnit, 1.0, 1.70724563015874e+25),
            (Unit::Karrat, Unit::Kilogram, 1.0, 0.0002),
            (Unit::Karrat, Unit::Gram, 4.0, 0.8),
            (Unit::Karrat, Unit::Milligram, 2.0, 400.0),
            (Unit::Karrat, Unit::Tonne, 2.0, 4.0000000000000003e-07),
            (Unit::Karrat, Unit::LongTon, 3.0, 5.9052438e-07),
            (Unit::Karrat, Unit::ShortTon, 4.0, 8.818497600000002e-07),
            (Unit::Karrat, Unit::Pound, 1.0, 0.00044092488404000004),
            (Unit::Karrat, Unit::Ounce, 2.0, 0.0141095962892),
            (Unit::Karrat, Unit::Karrat, 4.0, 4.0),
            (Unit::Karrat, Unit::AtomicMassUnit, 4.0, 4.8177093216e+23),
            (Unit::AtomicMassUnit, Unit::Kilogram, 4.0, 6.642160796e-27),
            (Unit::AtomicMassUnit, Unit::Gram, 2.0, 3.321080398e-24),
            (
                Unit::AtomicMassUnit,
                Unit::Milligram,
                2.0,
                3.3210803980000002e-21,
            ),
            (
                Unit::AtomicMassUnit,
                Unit::Tonne,
                3.0,
                4.9816205970000004e-30,
            ),
            (
                Unit::AtomicMassUnit,
                Unit::LongTon,
                3.0,
                4.9029473573977584e-30,
            ),
            (
                Unit::AtomicMassUnit,
                Unit::ShortTon,
                1.0,
                1.830433719948128e-30,
            ),
            (
                Unit::AtomicMassUnit,
                Unit::Pound,
                3.0,
                1.0982602420317504e-26,
            ),
            (
                Unit::AtomicMassUnit,
                Unit::Ounce,
                2.0,
                1.1714775914938915e-25,
            ),
            (Unit::AtomicMassUnit, Unit::Karrat, 2.0, 1.660540199e-23),
            (
                Unit::AtomicMassUnit,
                Unit::AtomicMassUnit,
                2.0,
                1.999999998903455,
            ),
        ];
        for (from_type, to_type, val, to_val) in PAIRS {
            assert_eq!(weight_conversion(*val, *from_type, *to_type), *to_val);
        }
    }
}
