// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// HashTable
// 字典计数
pub fn can_construct1(ransom_note: String, magazine: String) -> bool {
    debug_assert!(!ransom_note.is_empty());
    debug_assert!(!magazine.is_empty());

    // 统计字符数.
    let mut map: HashMap<char, usize> = HashMap::with_capacity(magazine.len());
    for chr in magazine.chars() {
        *map.entry(chr).or_default() += 1;
    }

    // 遍历 ransom_note, 并把相应的字符从字典中减去.
    for chr in ransom_note.chars() {
        match map.get_mut(&chr) {
            Some(count) if *count > 0 => *count -= 1,
            _ => return false,
        }
    }
    true
}

pub type SolutionFn = fn(String, String) -> bool;

fn check_solution(func: SolutionFn) {
    let ransom_note = "a".to_owned();
    let magazine = "b".to_owned();
    assert!(!func(ransom_note, magazine));

    let ransom_note = "aa".to_owned();
    let magazine = "ab".to_owned();
    assert!(!func(ransom_note, magazine));

    let ransom_note = "aa".to_owned();
    let magazine = "aab".to_owned();
    assert!(func(ransom_note, magazine));
}

fn main() {
    check_solution(can_construct1);
}

#[cfg(test)]
mod tests {
    use super::{can_construct1, check_solution};

    #[test]
    fn test_can_construct1() {
        check_solution(can_construct1);
    }
}
