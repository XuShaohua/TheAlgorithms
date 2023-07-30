// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Graham's Law of Effusion
//!
//! [Graham's law](https://en.wikipedia.org/wiki/Graham%27s_law)

#[must_use]
pub fn validate(values: &[f64]) -> bool {
    !values.is_empty() && values.iter().all(|value| *value > 0.0)
}

#[must_use]
pub fn effusion_ratio(molar_mass_1: f64, molar_mass_2: f64) -> f64 {
    debug_assert!(validate(&[molar_mass_1, molar_mass_2]));
    (molar_mass_2 / molar_mass_1).sqrt()
}

#[must_use]
pub fn first_effusion_rate(effusion_rate: f64, molar_mass_1: f64, molar_mass_2: f64) -> f64 {
    debug_assert!(validate(&[effusion_rate, molar_mass_1, molar_mass_2]));
    effusion_rate * (molar_mass_2 / molar_mass_1).sqrt()
}

#[must_use]
pub fn second_effusion_rate(effusion_rate: f64, molar_mass_1: f64, molar_mass_2: f64) -> f64 {
    debug_assert!(validate(&[effusion_rate, molar_mass_1, molar_mass_2]));
    effusion_rate / (molar_mass_2 / molar_mass_1).sqrt()
}

#[must_use]
pub fn first_molar_mass(molar_mass: f64, effusion_rate_1: f64, effusion_rate_2: f64) -> f64 {
    debug_assert!(validate(&[molar_mass, effusion_rate_1, effusion_rate_2]));
    molar_mass / (effusion_rate_1 / effusion_rate_2).powi(2)
}

#[must_use]
pub fn second_molar_mass(molar_mass: f64, effusion_rate_1: f64, effusion_rate_2: f64) -> f64 {
    debug_assert!(validate(&[molar_mass, effusion_rate_1, effusion_rate_2]));
    (effusion_rate_1 / effusion_rate_2).powi(2) / molar_mass
}

pub trait Round6 {
    #[must_use]
    fn round6(self) -> Self;
}

impl Round6 for f64 {
    fn round6(self) -> Self {
        (self * 1_000_000.0).round() / 1_000_000.0
    }
}

#[cfg(test)]
mod tests {

    use super::{
        effusion_ratio, first_effusion_rate, first_molar_mass, second_effusion_rate,
        second_molar_mass, validate, Round6,
    };

    #[test]
    fn test_validte() {
        assert!(validate(&[2.016, 4.002]));
        assert!(!validate(&[-2.016, 4.002]));
    }

    #[test]
    fn test_effusion_ratio() {
        assert_eq!(effusion_ratio(2.016, 4.002).round6(), 1.408943);
    }

    #[test]
    fn test_first_effusion_rate() {
        assert_eq!(first_effusion_rate(1.0, 2.016, 4.002).round6(), 1.408943);
    }

    #[test]
    fn test_second_effusion_rate() {
        assert_eq!(second_effusion_rate(1.0, 2.016, 4.002).round6(), 0.709752);
    }

    #[test]
    fn test_first_molar_mass() {
        assert_eq!(first_molar_mass(2.0, 1.408943, 0.709752).round6(), 0.507524);
    }

    #[test]
    fn test_second_molar_mass() {
        assert_eq!(
            second_molar_mass(2.0, 1.408943, 0.709752).round6(),
            1.970351
        );
    }
}
