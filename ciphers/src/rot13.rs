// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[must_use]
#[inline]
pub fn rot13(s: &str) -> String {
    rot_any(s, 13)
}

/// Rotate in character table.
///
/// See `https://en.wikipedia.org/wiki/ROT13`
#[must_use]
pub fn rot_any(s: &str, n: u32) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='z' => (b'a' + ((c as u32 + n - 'a' as u32) % 26) as u8).into(),
            'A'..='Z' => (b'A' + ((c as u32 + n - 'A' as u32) % 26) as u8).into(),
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::rot13;

    #[test]
    fn test_long_msg() {
        let msg = "My secret bank account number is 173-52946 so don't tell anyone!!";
        let out = rot13(msg);
        let expected = "Zl frperg onax nppbhag ahzore vf 173-52946 fb qba'g gryy nalbar!!";
        assert_eq!(out, expected);
    }

    #[test]
    fn test_twice() {
        assert_eq!("Hello", rot13(&rot13("Hello")));
    }

    #[test]
    fn test_utf8() {
        assert_eq!("Hi, 你好", rot13("Uv, 你好"));
    }
}
