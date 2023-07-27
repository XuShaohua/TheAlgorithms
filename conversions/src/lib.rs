// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![deny(
    warnings,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic
)]

mod binary_to_decimal;
mod binary_to_hexadecimal;
mod binary_to_octal;
pub mod error;
mod excel_title_to_column;
mod hex_to_bin;

pub use binary_to_decimal::binary_to_decimal;
pub use binary_to_hexadecimal::binary_to_hexadecimal;
pub use binary_to_octal::binary_to_octal;
pub use excel_title_to_column::excel_title_to_column;
pub use hex_to_bin::hex_to_bin;

pub mod astronomical_length;
pub mod energy;
pub mod length;
pub mod molecular_chemistry;
pub mod pressure;
pub mod rgb_hsv;
pub mod roman_numerals;
pub mod speed;
pub mod temperature;
pub mod volume;
pub mod weight;
