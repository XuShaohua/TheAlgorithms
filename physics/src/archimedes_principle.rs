// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Calculates buoyant force on object submerged within static fluid.
//! Discovered by greek mathematician, Archimedes.
//!
//! [ Archimedes principle](https://en.wikipedia.org/wiki/Archimedes%27_principle)

pub const G: f64 = 9.80665;

/// Get buoyant force on object in Newtons
#[must_use]
pub fn archimedes_principle(fluid_density: f64, volume: f64, gravity: f64) -> f64 {
    debug_assert!(fluid_density > 0.0);
    debug_assert!(volume >= 0.0);
    debug_assert!(gravity > 0.0);
    fluid_density * gravity * volume
}

#[cfg(test)]
mod tests {
    use super::{archimedes_principle, G};

    #[test]
    fn test_archimedes_principle() {
        assert_eq!(archimedes_principle(997.0, 0.5, 9.8), 4885.3);
        assert_eq!(archimedes_principle(997.0, 0.7, G), 6_844.061_035);
    }
}
