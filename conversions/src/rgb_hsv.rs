// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

//! Conversion between RGB and HSV color models.
//!
//! # References
//! - [RGB](https://en.wikipedia.org/wiki/RGB_color_model)
//! - [HSL and HSV](https://en.wikipedia.org/wiki/HSL_and_HSV))

#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv(f64, f64, f64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb(u8, u8, u8);

/// Conversion from the HSV-representation to the RGB-representation.
#[must_use]
pub fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> (u8, u8, u8) {
    debug_assert!((0.0..=360.0).contains(&hue));
    debug_assert!((0.0..=1.0).contains(&saturation));
    debug_assert!((0.0..=1.0).contains(&value));

    let chroma = value * saturation;
    let hue_section = hue / 60.0;
    let second_largest_component = chroma * (1.0 - (hue_section % 2.0 - 1.0).abs());
    let match_value = value - chroma;

    let red: f64;
    let green: f64;
    let blue: f64;
    if (0.0..=1.0).contains(&hue_section) {
        red = 255.0 * (chroma + match_value);
        green = 255.0 * (second_largest_component + match_value);
        blue = 255.0 * (match_value);
    } else if hue_section > 1.0 && hue_section <= 2.0 {
        red = 255.0 * (second_largest_component + match_value);
        green = 255.0 * (chroma + match_value);
        blue = 255.0 * (match_value);
    } else if hue_section > 2.0 && hue_section <= 3.0 {
        red = 255.0 * (match_value);
        green = 255.0 * (chroma + match_value);
        blue = 255.0 * (second_largest_component + match_value);
    } else if hue_section > 3.0 && hue_section <= 4.0 {
        red = 255.0 * (match_value);
        green = 255.0 * (second_largest_component + match_value);
        blue = 255.0 * (chroma + match_value);
    } else if hue_section > 4.0 && hue_section <= 5.0 {
        red = 255.0 * (second_largest_component + match_value);
        green = 255.0 * (match_value);
        blue = 255.0 * (chroma + match_value);
    } else {
        red = 255.0 * (chroma + match_value);
        green = 255.0 * match_value;
        blue = 255.0 * (second_largest_component + match_value);
    }

    (red.round() as u8, green.round() as u8, blue.round() as u8)
}

/// Conversion from the RGB-representation to the HSV-representation.
#[must_use]
pub fn rgb_to_hsv(red: u8, green: u8, blue: u8) -> (f64, f64, f64) {
    let float_red = f64::from(red) / 255.0;
    let float_green = f64::from(green) / 255.0;
    let float_blue = f64::from(blue) / 255.0;
    let value: f64 = float_red.max(float_green.max(float_blue));
    let chroma: f64 = value - float_red.min(float_green.min(float_blue));
    let saturation: f64 = if value == 0.0 { 0.0 } else { chroma / value };

    let hue: f64 = if chroma == 0.0 {
        0.0
    } else if (value - float_red).abs() < f64::EPSILON {
        60.0_f64 * (0.0 + (float_green - float_blue) / chroma)
    } else if (value - float_green).abs() < f64::EPSILON {
        60.0_f64 * (2.0 + (float_blue - float_red) / chroma)
    } else {
        60.0_f64 * (4.0 + (float_red - float_green) / chroma)
    };

    let hue: f64 = (hue + 360.0) % 360.0;
    (hue, saturation, value)
}

/// Utility-function to check that two hsv-colors are approximately equal
#[must_use]
pub fn approximately_equal_hsv(hsv_1: (f64, f64, f64), hsv_2: (f64, f64, f64)) -> bool {
    let check_hue = (hsv_1.0 - hsv_2.0).abs() < 0.2;
    let check_saturation = (hsv_1.1 - hsv_2.1).abs() < 0.002;
    let check_value = (hsv_1.2 - hsv_2.2).abs() < 0.002;

    check_hue && check_saturation && check_value
}

#[cfg(test)]
mod tests {
    use super::{approximately_equal_hsv, hsv_to_rgb, rgb_to_hsv};

    #[test]
    fn test_hsv_to_rgb() {
        const PAIRS: &[((f64, f64, f64), (u8, u8, u8))] = &[
            ((0.0, 0.0, 0.0), (0, 0, 0)),
            ((0.0, 0.0, 1.0), (255, 255, 255)),
            ((0.0, 1.0, 1.0), (255, 0, 0)),
            ((60.0, 1.0, 1.0), (255, 255, 0)),
            ((120.0, 1.0, 1.0), (0, 255, 0)),
            ((240.0, 1.0, 1.0), (0, 0, 255)),
            ((300.0, 1.0, 1.0), (255, 0, 255)),
            ((180.0, 0.5, 0.5), (64, 128, 128)),
            ((234.0, 0.14, 0.88), (193, 196, 224)),
            ((330.0, 0.75, 0.5), (128, 32, 80)),
        ];
        for ((h, s, v), rgb) in PAIRS {
            assert_eq!(&hsv_to_rgb(*h, *s, *v), rgb);
        }
    }

    #[test]
    fn test_rgb_to_hsvg() {
        const PAIRS: &[((u8, u8, u8), (f64, f64, f64))] = &[
            ((0, 0, 0), (0.0, 0.0, 0.0)),
            ((255, 255, 255), (0.0, 0.0, 1.0)),
            ((255, 0, 0), (0.0, 1.0, 1.0)),
            ((255, 255, 0), (60.0, 1.0, 1.0)),
            ((0, 255, 0), (120.0, 1.0, 1.0)),
            ((0, 0, 255), (240.0, 1.0, 1.0)),
            ((255, 0, 255), (300.0, 1.0, 1.0)),
            ((64, 128, 128), (180.0, 0.5, 0.5)),
            ((193, 196, 224), (234.0, 0.14, 0.88)),
            ((128, 32, 80), (330.0, 0.75, 0.5)),
        ];
        for ((r, g, b), hsv) in PAIRS {
            assert!(approximately_equal_hsv(rgb_to_hsv(*r, *g, *b), *hsv));
        }
    }
}
