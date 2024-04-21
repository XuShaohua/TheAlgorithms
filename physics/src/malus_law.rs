// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Finding the intensity of light transmitted through a polariser using Malus Law.
//!
//! [Malus's law](https://en.wikipedia.org/wiki/Polarizer#Malus's_law_and_other_properties)

#[must_use]
pub fn malus_law(initial_intensity: f64, angle: f64) -> f64 {
    debug_assert!(initial_intensity >= 0.0);
    debug_assert!((0.0..=360.0).contains(&angle));
    initial_intensity * (angle.to_radians().cos()).powi(2)
}

#[cfg(test)]
mod tests {
    use super::malus_law;

    #[test]
    fn test_malus_law() {
        const PAIRS: &[(f64, f64, f64)] = &[
            (10.0, 45.0, 5.0),
            (100.0, 60.0, 25.0),
            (50.0, 150.0, 37.5),
            (75.0, 270.0, 0.0),
            (100.0, 180.0, 100.0),
            (100.0, 360.0, 100.0),
        ];
        for (initial_intensity, angle, result) in PAIRS {
            let intensity = malus_law(*initial_intensity, *angle);
            let round_intensity = (intensity * 10.0).round() / 10.0;
            assert!((round_intensity - result).abs() < f64::EPSILON);
        }
    }
}
