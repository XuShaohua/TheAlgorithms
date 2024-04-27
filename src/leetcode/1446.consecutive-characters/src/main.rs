// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn max_power(s: String) -> i32 {
    let mut last_char = '0';
    let mut max_chars = 0;
    let mut chars_count = 0;
    for c in s.chars() {
        if c == last_char {
            chars_count += 1;
        } else {
            max_chars = max_chars.max(chars_count);
            chars_count = 1;
            last_char = c;
        }
    }
    max_chars.max(chars_count)
}

fn check_solution() {
    let s = "leetcode".to_owned();
    assert_eq!(max_power(s), 2);
    let s = "abbcccddddeeeeedcba".to_owned();
    assert_eq!(max_power(s), 5);
}

fn main() {
    check_solution();
}
