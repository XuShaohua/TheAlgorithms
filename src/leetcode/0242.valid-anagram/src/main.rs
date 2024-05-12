// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// 数组排序
pub fn is_anagram1(s: String, t: String) -> bool {
    debug_assert!(!s.is_empty());
    debug_assert!(!t.is_empty());
    if s.len() != t.len() {
        return false;
    }

    let mut s: Vec<char> = s.chars().collect();
    s.sort();
    let mut t: Vec<char> = t.chars().collect();
    t.sort();

    s == t
}

// 数组原地排序
// 不重新分配内存
// 只适合 ascii, 不适合 unicode
pub fn is_anagram2(s: String, t: String) -> bool {
    debug_assert!(!s.is_empty());
    debug_assert!(!t.is_empty());
    if s.len() != t.len() {
        return false;
    }

    let mut s = s;
    let mut t = t;
    unsafe {
        s.as_mut_vec().sort_unstable();
        t.as_mut_vec().sort_unstable();
    }
    s == t
}

// HashTable
// 字典记数
pub fn is_anagram3(s: String, t: String) -> bool {
    debug_assert!(!s.is_empty());
    debug_assert!(!t.is_empty());
    if s.len() != t.len() {
        return false;
    }

    // 构造字典
    let mut map_s: HashMap<char, usize> = HashMap::with_capacity(s.len());
    for chr in s.chars() {
        *map_s.entry(chr).or_default() += 1;
    }
    for chr in t.chars() {
        match map_s.get_mut(&chr) {
            Some(count) if *count > 0 => *count -= 1,
            _ => return false,
        }
    }

    true
}

pub type SolutionFn = fn(String, String) -> bool;

fn check_solution(func: SolutionFn) {
    let s = "anagram".to_owned();
    let t = "nagaram".to_owned();
    assert!(func(s, t));

    let s = "rat".to_owned();
    let t = "car".to_owned();
    assert!(!func(s, t));
}

fn main() {
    check_solution(is_anagram1);
    check_solution(is_anagram2);
    check_solution(is_anagram3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_anagram1, is_anagram2, is_anagram3};

    #[test]
    fn test_is_anagram1() {
        check_solution(is_anagram1);
    }

    #[test]
    fn test_is_anagram2() {
        check_solution(is_anagram2);
    }

    #[test]
    fn test_is_anagram3() {
        check_solution(is_anagram3);
    }
}
