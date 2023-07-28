// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//!  The root-mean-square speed is essential in measuring the average speed of particles
//! contained in a gas

const UNIVERSAL_GAS_CONSTANT: f64 = 8.314_459_8;

#[must_use]
pub fn rms_speed_of_molecule(temperature: f64, molar_mass: f64) -> f64 {
    debug_assert!(temperature >= 0.0);
    debug_assert!(molar_mass > 0.0);
    (3.0 * UNIVERSAL_GAS_CONSTANT * temperature / molar_mass).sqrt()
}

#[cfg(test)]
mod tests {
    use super::rms_speed_of_molecule;

    #[test]
    fn test_rms_speed_of_molecule() {
        assert_eq!(rms_speed_of_molecule(100.0, 2.0), 35.315279554323226);
        assert_eq!(rms_speed_of_molecule(273.0, 12.0), 23.821458421977443);
    }
}
