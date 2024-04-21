// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Calculate the speed of sound.
//!
//! For fluids in general, the speed of sound c is given by the Newton–Laplace equation:
//! ```txt
//!  c_fluid = sqrt(K_s * p)
//! ```
//! Where:
//! - `c_fluid`: speed of sound in fluid
//! - `K_s`: isentropic bulk modulus
//! - `p`: density of fluid
//!
//! [Speed of sound](https://en.wikipedia.org/wiki/Speed_of_sound)

/// This method calculates the speed of sound in fluid
#[must_use]
pub fn speed_in_fluid(density: f64, bulk_modulus: f64) -> f64 {
    debug_assert!(density > 0.0);
    debug_assert!(bulk_modulus > 0.0);
    (bulk_modulus / density).sqrt()
}

#[cfg(test)]
mod tests {
    use super::speed_in_fluid;

    #[test]
    fn test_speed_in_fluid() {
        assert!(
            (speed_in_fluid(998.0, 2.15 * 10.0_f64.powi(9)) - 1_467.756_320_795_270_5).abs()
                < f64::EPSILON
        );
        assert!(
            (speed_in_fluid(13600.0, 28.5 * 10.0_f64.powi(9)) - 1_447.614_670_861_731).abs()
                < f64::EPSILON
        );
    }
}
