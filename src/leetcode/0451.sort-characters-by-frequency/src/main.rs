// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::collections::HashMap;

// HashTable
// 字典计数 + 有序数组
pub fn frequency_sort1(s: String) -> String {
    assert!(!s.is_empty());

    // 计数
    let mut map: HashMap<char, usize> = HashMap::new();
    for chr in s.chars() {
        //map.entry(chr).and_modify(|count| *count += 1).or_insert(1);
        *map.entry(chr).or_default() += 1;
    }

    // 排序, 构造有序数组.
    let mut chars_count: Vec<(char, usize)> = map.into_iter().collect();
    // 根据 count 降序排列.
    //chars_count.sort_by(|a, b| b.1.cmp(&a.1));
    chars_count.sort_by(|a, b| {
        match b.1.cmp(&a.1) {
            Ordering::Equal => {
                // 次级排序, 按字符升序排.
                a.0.cmp(&b.0)
            }
            order => order,
        }
    });

    // 重新拼装字符串
    let mut out = String::with_capacity(s.len());
    for (chr, count) in chars_count {
        for _i in 0..count {
            out.push(chr);
        }
    }

    out
}

// TODO(Shaohua): Priority Queue

pub type SolutionFn = fn(String) -> String;

fn check_solution(func: SolutionFn) {
    let s = "tree".to_owned();
    assert_eq!(func(s), "eert".to_owned());

    let s = "cccaaa".to_owned();
    assert_eq!(func(s), "aaaccc".to_owned());

    let s = "Aabb".to_owned();
    assert_eq!(func(s), "bbAa".to_owned());
}

fn main() {
    check_solution(frequency_sort1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, frequency_sort1};

    #[test]
    fn test_frequency_sort1() {
        check_solution(frequency_sort1);
    }
}
