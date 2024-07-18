// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Convert a string to sequence of numbers
/// corresponding to the character's  position in the alphabet.
#[must_use]
pub fn encode(s: &str) -> Vec<u8> {
    s.as_bytes().iter().map(|byte| byte - 96).collect()
}

#[must_use]
pub fn decode(slice: &[u8]) -> String {
    slice.iter().map(|byte| (byte + 96) as char).collect()
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};

    #[test]
    fn test_encode() {
        assert_eq!(encode("myname"), &[13, 25, 14, 1, 13, 5]);
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode(&[13, 25, 14, 1, 13, 5]), "myname");
    }
}
