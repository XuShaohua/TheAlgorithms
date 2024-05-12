// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

// HashTable + HashSet
// 一对一映射
pub fn is_isomorphic1(s: String, t: String) -> bool {
    debug_assert!(s.len() == t.len());
    debug_assert!(!s.is_empty());

    // 使用 map + set, 实现一对一的映射
    // 字符 s->t 的映射表
    let mut map: HashMap<char, char> = HashMap::with_capacity(s.len());
    // t 中的字符
    let mut set: HashSet<char> = HashSet::with_capacity(t.len());

    for (chr_s, chr_t) in s.chars().zip(t.chars()) {
        match map.entry(chr_s) {
            // 检查是不是映射到别的字符了
            Entry::Occupied(entry) if *entry.get() != chr_t => return false,
            Entry::Vacant(entry) => {
                // 检查新加入的映射是不是一对一的.
                if set.contains(&chr_t) {
                    return false;
                }
                entry.insert(chr_t);
                set.insert(chr_t);
            }
            _ => (),
        }
    }
    true
}

// HashTable
// 使用两个 HashTable
pub fn is_isomorphic2(s: String, t: String) -> bool {
    debug_assert!(s.len() == t.len());
    debug_assert!(!s.is_empty());

    // 使用 map + set, 实现一对一的映射
    // 字符 s->t 的映射表
    let mut map: HashMap<char, char> = HashMap::with_capacity(s.len());
    // t 中的字符
    let mut set: HashSet<char> = HashSet::with_capacity(t.len());

    for (chr_s, chr_t) in s.chars().zip(t.chars()) {
        match map.entry(chr_s) {
            // 检查是不是映射到别的字符了
            Entry::Occupied(entry) if *entry.get() != chr_t => return false,
            Entry::Vacant(entry) => {
                // 检查新加入的映射是不是一对一的.
                if set.contains(&chr_t) {
                    return false;
                }
                entry.insert(chr_t);
                set.insert(chr_t);
            }
            _ => (),
        }
    }
    true
}

pub type SolutionFn = fn(String, String) -> bool;

fn check_solution(func: SolutionFn) {
    let s = "egg".to_owned();
    let t = "add".to_owned();
    assert!(func(s, t));

    let s = "foo".to_owned();
    let t = "bar".to_owned();
    assert!(!func(s, t));

    let s = "paper".to_owned();
    let t = "title".to_owned();
    assert!(func(s, t));

    let s = "bbbaaaba".to_owned();
    let t = "aaabbbba".to_owned();
    assert!(!func(s, t));

    let s = "badc".to_owned();
    let t = "baba".to_owned();
    assert!(!func(s, t));
}

fn main() {
    check_solution(is_isomorphic1);
    check_solution(is_isomorphic2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_isomorphic1, is_isomorphic2};

    #[test]
    fn test_is_isomorphic1() {
        check_solution(is_isomorphic1);
    }

    #[test]
    fn test_is_isomorphic2() {
        check_solution(is_isomorphic2);
    }
}
