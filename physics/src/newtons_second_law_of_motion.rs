// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Newton's second law of motion pertains to the behavior of objects for which
//! all existing forces are not balanced.
//!
//! [Newtons Second Law](https://www.physicsclassroom.com/class/newtlaws/Lesson-3/Newton-s-Second-Law)

#[must_use]
pub fn newtons_second_law_of_motion(mass: f64, acceleration: f64) -> f64 {
    mass * acceleration
}

#[cfg(test)]
mod tests {

    use super::newtons_second_law_of_motion;

    #[test]
    fn test_newtons_second_law_of_motion() {
        assert_eq!(newtons_second_law_of_motion(10.0, 10.0), 100.0);
        assert_eq!(newtons_second_law_of_motion(2.0, 1.0), 2.0);
    }
}
