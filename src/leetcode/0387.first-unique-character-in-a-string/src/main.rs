// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// Hash table
// 用哈稀表来计数
pub fn first_uniq_char1(s: String) -> i32 {
    // 先生成计数字典.
    let mut map: HashMap<char, usize> = HashMap::new();
    for chr in s.chars() {
        map.entry(chr).and_modify(|count| *count += 1).or_insert(1);
    }

    // 再遍历字符串, 找到第一个计数为1的字符.
    for (index, chr) in s.chars().enumerate() {
        if let Some(count) = map.get(&chr) {
            if *count == 1 {
                return index as i32;
            }
        }
    }
    -1
}

// Frequency Array
// 使用 [usize; 26] 来计数, 替换上面的哈稀表
pub fn first_uniq_char2(s: String) -> i32 {
    // 先生成各个字符出现的频率.
    let mut freq = [0_usize; 26];
    for byte in s.as_bytes() {
        let index = (byte - b'a') as usize;
        freq[index] += 1;
    }

    // 再遍历字符串, 找到频率为1的那个字符.
    for (index, byte) in s.as_bytes().iter().enumerate() {
        let byte_index = (byte - b'a') as usize;
        if freq[byte_index] == 1 {
            return index as i32;
        }
    }

    -1
}

// Queue
// TODO(Shaohua):

pub type SolutionFn = fn(String) -> i32;

fn check_solution(func: SolutionFn) {
    let s = "leetcode".to_owned();
    assert_eq!(func(s), 0);

    let s = "loveleetcode".to_owned();
    assert_eq!(func(s), 2);

    let s = "aabb".to_owned();
    assert_eq!(func(s), -1);
}

fn main() {
    check_solution(first_uniq_char1);
    check_solution(first_uniq_char2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, first_uniq_char1, first_uniq_char2};

    #[test]
    fn test_first_uniq_char1() {
        check_solution(first_uniq_char1);
    }

    #[test]
    fn test_first_uniq_char2() {
        check_solution(first_uniq_char2);
    }
}
