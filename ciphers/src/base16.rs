// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Write;

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

pub fn base16_decode() {}

#[cfg(test)]
mod tests {
    use super::base16_encode;

    #[test]
    fn test_encode() {
        assert_eq!(base16_encode(b"Hello World!"), "48656C6C6F20576F726C6421");
        assert_eq!(base16_encode(b"HELLO WORLD!"), "48454C4C4F20574F524C4421");
        assert_eq!(base16_encode(b""), "");
    }
}
