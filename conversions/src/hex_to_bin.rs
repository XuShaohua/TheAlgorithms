// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::error::HexError;

/// Convert a hexadecimal value to its binary equivalent
///
/// # Errors
/// Raise error if `hex_num` is invalid
pub fn hex_to_bin(hex_num: &str) -> Result<i64, HexError> {
    let hex_num = hex_num.trim();
    if hex_num.is_empty() {
        return Err(HexError::Empty);
    }
    if !hex_num.is_ascii() {
        return Err(HexError::NonAscii);
    }

    let hex_bytes = hex_num.as_bytes();
    let is_negative = hex_bytes[0] == b'-';
    println!("negative: {is_negative}");

    let int_num = i64::from_str_radix(hex_num, 16).map_err(|_err| HexError::InvalidValue)?;
    println!("int_num: {int_num}");

    /*
    bin_str = ""
    while int_num > 0 {
        bin_str = str(int_num % 2) + bin_str;
        int_num >>= 1;
    }

    let mut bin = bin_str::parse().unwrap();
    if is_negative {
        bin *= -1;
    }
    Ok(bin)
    */
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::hex_to_bin;

    #[test]
    fn test_hex_to_bin() {
        assert_eq!(hex_to_bin("AC"), Ok(10101100));
        assert_eq!(hex_to_bin("9A4"), Ok(100110100100));
        assert_eq!(hex_to_bin("   12f   "), Ok(100101111));
        assert_eq!(hex_to_bin("FfFf"), Ok(1111111111111111));
        assert_eq!(hex_to_bin("-fFfF"), Ok(-1111111111111111));
    }
}
