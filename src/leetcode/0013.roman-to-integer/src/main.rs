// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// 哈稀表
pub fn roman_to_int1(s: String) -> i32 {
    assert!(!s.is_empty());

    // 使用哈稀表处理罗马数字到阿拉伯数字的映射.
    let map = HashMap::<u8, i32>::from([
        (b'I', 1),
        (b'V', 5),
        (b'X', 10),
        (b'L', 50),
        (b'C', 100),
        (b'D', 500),
        (b'M', 1000),
    ]);

    let bytes = s.as_bytes();
    let mut sum = 0;

    for i in 0..bytes.len() {
        // 获取当前元素的数值
        let val: i32 = *map.get(&bytes[i]).unwrap();

        // 获取下个元素的数值
        let next_val: i32 = if i + 1 < bytes.len() {
            *map.get(&bytes[i + 1]).unwrap()
        } else {
            0
        };

        // 根据val, next_val 的关系, 确定采用加法还是减法
        if val >= next_val {
            // 如果是加法
            sum += val;
        } else {
            // 如果是减法
            sum -= val;
        }
    }

    sum
}

// 模式匹配
// 比哈稀表快, 没有分配堆内存
pub fn roman_to_int2(s: String) -> i32 {
    // 辅助函数, 处理罗马数字到阿拉伯数字的映射.
    #[must_use]
    #[inline]
    const fn get_roman_number(symbol: char) -> i32 {
        match symbol {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    assert!(!s.is_empty());
    let mut sum = 0;
    let mut curr_val = 0;
    let mut prev_val = 0;

    for symbol in s.chars() {
        // 获取当前元素的数值
        curr_val = get_roman_number(symbol);

        // 根据val, next_val 的关系, 确定采用加法还是减法
        if curr_val > prev_val {
            // 如果是减法
            sum -= prev_val;
        } else {
            // 如果是加法
            sum += prev_val;
        }
        prev_val = curr_val;
    }

    // 别忘了加上最后一个数值.
    sum += curr_val;

    sum
}

pub type SolutionFn = fn(String) -> i32;

fn check_solution(func: SolutionFn) {
    const RECORDS: &[(&str, i32)] = &[("III", 3), ("LVIII", 58), ("MCMXCIV", 1994)];

    for record in RECORDS {
        assert_eq!(func(record.0.to_owned()), record.1);
    }
}

fn main() {
    check_solution(roman_to_int1);
    check_solution(roman_to_int2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, roman_to_int1, roman_to_int2};

    #[test]
    fn test_solution1() {
        check_solution(roman_to_int1);
    }

    #[test]
    fn test_solution2() {
        check_solution(roman_to_int2);
    }
}
