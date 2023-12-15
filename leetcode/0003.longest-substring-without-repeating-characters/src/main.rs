// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::collections::HashSet;

fn solution1(s: String) -> i32 {
    let mut chars = HashSet::with_capacity(s.len());
    let mut max_chars: usize = 0;
    for c in s.chars() {
        if let Some(&_found_c) = chars.get(&c) {
            max_chars = max_chars.max(chars.len());
            chars.clear();
        }
        chars.insert(c);
    }
    max_chars as i32
}

fn check_solution1() {
    let s = "abcabcbb".to_owned();
    assert_eq!(solution1(s), 3);

    let s = "bbbbb".to_owned();
    assert_eq!(solution1(s), 1);

    let s = "pwwkew".to_owned();
    assert_eq!(solution1(s), 3);
}

fn main() {
    check_solution1();
}

#[cfg(test)]
mod tests {
    use super::check_solution1;

    #[test]
    fn test_solution1() {
        check_solution1();
    }
}
