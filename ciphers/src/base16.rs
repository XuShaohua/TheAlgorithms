// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use std::fmt::{self, Write};

/// Encodes the given bytes into base16.
#[must_use]
pub fn base16_encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        // It's safe to unwrap() the result.
        write!(&mut out, "{byte:02X}").unwrap();
    }
    out.shrink_to_fit();
    out
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DecodeError {
    OddNumberOfDigits,
    Lowercase,
    InvalidCharacter,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OddNumberOfDigits => "Contains odd number of hex digits",
            Self::Lowercase => "Got lowercase of character",
            Self::InvalidCharacter => "Invalid character",
        };
        write!(f, "{s}")
    }
}

impl std::error::Error for DecodeError {}

/// Decodes the given base16 encoded data into bytes.
///
/// # Errors
///
/// Returns error if str contains invalid hex digit chars.
pub fn base16_decode(s: &str) -> Result<Vec<u8>, DecodeError> {
    // Check data validity, following RFC3548
    // https://www.ietf.org/rfc/rfc3548.txt
    if s.len().is_multiple_of(2) {
        return Err(DecodeError::OddNumberOfDigits);
    }

    // Check the character set - the standard base16 alphabet
    // is uppercase according to RFC3548 section 6
    for c in s.chars() {
        match c {
            '0'..='9' | 'A'..='F' => (),
            'a'..='f' => return Err(DecodeError::Lowercase),
            _ => return Err(DecodeError::InvalidCharacter),
        }
    }

    // For every two hexadecimal digits (= a byte), turn it into an integer.
    // Then, string the result together into bytes, and return it.
    let mut bytes = Vec::with_capacity(s.len() / 2);
    for chunk in s.as_bytes().chunks(2) {
        let high = match chunk[0] {
            b @ b'0'..=b'9' => b - b'0',
            b => b - b'A' + 10,
        };
        let low = match chunk[1] {
            b @ b'0'..=b'9' => b - b'0',
            b => b - b'A' + 10,
        };
        let byte = (high << 4) + low;
        bytes.push(byte);
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::{base16_decode, base16_encode, DecodeError};

    #[test]
    fn test_encode() {
        assert_eq!(base16_encode(b"Hello World!"), "48656C6C6F20576F726C6421");
        assert_eq!(base16_encode(b"HELLO WORLD!"), "48454C4C4F20574F524C4421");
        assert_eq!(base16_encode(b""), "");
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            base16_decode("48656C6C6F20576F726C6421"),
            Ok(b"Hello World!".to_vec())
        );
        assert_eq!(
            base16_decode("48454C4C4F20574F524C4421"),
            Ok(b"HELLO WORLD!".to_vec())
        );
        assert_eq!(base16_decode(""), Ok(b"".to_vec()));
    }

    #[test]
    fn test_decode_error() {
        assert_eq!(base16_decode("486"), Err(DecodeError::OddNumberOfDigits));
        assert_eq!(
            base16_decode("48656c6c6f20576f726c6421"),
            Err(DecodeError::Lowercase)
        );
        assert_eq!(
            base16_decode("THIS is not base64 encoded data."),
            Err(DecodeError::InvalidCharacter)
        );
    }
}
