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
pub use binary_to_decimal::binary_to_decimal;
mod binary_to_hexadecimal;
pub use binary_to_hexadecimal::binary_to_hexadecimal;
mod binary_to_octal;
pub use binary_to_octal::binary_to_octal;
mod excel_title_to_column;
pub use excel_title_to_column::excel_title_to_column;
pub mod energy;
pub mod molecular_chemistry;
pub mod speed;
pub mod volume;

pub mod error;
