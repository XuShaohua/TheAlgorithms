// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::collections::HashSet;

// 滑动窗口法
pub fn length_of_longest_substring1(s: String) -> i32 {
    // 当窗口右侧经过一个字符时, 要检查它是不是在窗口内.
    // 如果不在窗口内, 则继续当窗口右侧右移;
    // 如果在窗口内了, 说明需要将窗口左侧向右移, 直到遇到相同的字符.
    // 然后计算最长的那个窗口.

    let mut set = HashSet::new();
    let bytes = s.as_bytes();

    let mut substring_max_len = 0;
    let mut left = 0;
    let mut right = 0;

    // 遍历所有字符.
    while right < bytes.len() {
        while left < right && set.contains(&bytes[right]) {
            set.remove(&bytes[left]);
            left += 1;
        }

        // 将窗口右侧字符加进去.
        set.insert(bytes[right]);
        right += 1;

        // 并更新最大字串.
        substring_max_len = substring_max_len.max(set.len());
    }

    // 返回结果
    substring_max_len as i32
}

pub type Func = fn(String) -> i32;

fn check_solution(func: Func) {
    let s = "abcabcbb".to_owned();
    assert_eq!(func(s), 3);

    let s = "bbbbb".to_owned();
    assert_eq!(func(s), 1);

    let s = "pwwkew".to_owned();
    assert_eq!(func(s), 3);
}

fn main() {
    check_solution(length_of_longest_substring1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, length_of_longest_substring1};

    #[test]
    fn test_solution1() {
        check_solution(length_of_longest_substring1);
    }
}
