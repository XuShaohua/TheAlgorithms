// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// A valid IP address must be four octets in the form of A.B.C.D,
/// where A,B,C and D are numbers from 0-254.
///
/// for example: 192.168.23.1, 172.254.254.254 are valid IP address
///              192.168.255.0, 255.192.3.121 are invalid IP address
#[must_use]
pub fn is_ipv4_valid(s: &str) -> bool {
    let mut parts = s.split('.');
    if parts.clone().count() != 4 {
        return false;
    }
    parts.all(|part| {
        part.parse::<i32>()
            .map_or(false, |num| (0..=255).contains(&num))
    })
}

#[cfg(test)]
mod tests {
    use super::is_ipv4_valid;

    #[test]
    fn test_is_ipv4_valid() {
        const LIST: &[(&str, bool)] = &[
            ("192.168.0.23", true),
            ("192.255.15.8", true),
            ("172.100.0.8", true),
            ("254.255.0.255", true),
            ("1.2.33333333.4", false),
            ("1.2.-3.4", false),
            ("1.2.3", false),
            ("1.2.3.4.5", false),
            ("1.2.A.4", false),
            ("0.0.0.0", true),
            ("1.2.3.", false),
        ];
        for (s, valid) in LIST {
            assert_eq!(is_ipv4_valid(s), *valid);
        }
    }
}
