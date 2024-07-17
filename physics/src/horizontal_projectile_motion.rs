// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Horizontal Projectile Motion problem in physics.
//!
//! [Projectile motion](https://en.wikipedia.org/wiki/Projectile_motion)

/// Acceleration Constant on Earth (unit m/s^2)
pub const G: f64 = 9.80665;

// Check that the arguments are valid
#[must_use]
fn check_args(init_velocity: f64, angle: f64) -> bool {
    init_velocity >= 0.0 && (1.0..=90.0).contains(&angle)
}

/// Returns the horizontal distance that the object cover
#[must_use]
pub fn horizontal_distance(init_velocity: f64, angle: f64) -> f64 {
    debug_assert!(check_args(init_velocity, angle));
    let radians = (2.0 * angle).to_radians();
    init_velocity.powi(2) * (radians).sin() / G
}

/// Returns the maximum height that the object reach
#[must_use]
pub fn max_height(init_velocity: f64, angle: f64) -> f64 {
    debug_assert!(check_args(init_velocity, angle));
    let radians = angle.to_radians();
    init_velocity.powi(2) * radians.sin().powi(2) / (2.0 * G)
}

/// Returns total time of the motion
#[must_use]
pub fn total_time(init_velocity: f64, angle: f64) -> f64 {
    debug_assert!(check_args(init_velocity, angle));
    let radians = angle.to_radians();
    2.0 * init_velocity * radians.sin() / G
}

pub trait Round2 {
    #[must_use]
    fn round2(self) -> Self;
}

impl Round2 for f64 {
    fn round2(self) -> Self {
        (self * 100.0).round() / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::{horizontal_distance, max_height, total_time, Round2};

    #[test]
    fn test_horizontal_distance() {
        assert!((horizontal_distance(30.0, 45.0).round2() - 91.77).abs() < f64::EPSILON);
        assert!((horizontal_distance(100.0, 78.0).round2() - 414.76).abs() < f64::EPSILON);
    }

    #[test]
    fn test_max_height() {
        assert!((max_height(30.0, 45.0).round2() - 22.94).abs() < f64::EPSILON);
        assert!((max_height(100.0, 78.0).round2() - 487.82).abs() < f64::EPSILON);
    }

    #[test]
    fn test_total_time() {
        assert!((total_time(30.0, 45.0).round2() - 4.33).abs() < f64::EPSILON);
        assert!((total_time(100.0, 78.0).round2() - 19.95).abs() < f64::EPSILON);
    }
}
