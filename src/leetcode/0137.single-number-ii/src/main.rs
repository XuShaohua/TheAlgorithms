// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::collections::HashMap;

// map, brute force
// 使用字典来统计数值出现的次数
pub fn single_number1(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for &num in &nums {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }
    for (num, count) in map {
        if count == 1 {
            return num;
        }
    }
    -1
}

// bit vector
pub fn single_number2(nums: Vec<i32>) -> i32 {
    const DIGIT_LEN: usize = 32;

    let mut ans = 0;
    // 遍历所有比特位
    for i in 0..DIGIT_LEN {
        // 计数所有整数在该比特位处的和
        let sum = nums.iter().map(|num| num >> i & 1).sum::<i32>();
        // bit 的值就是落单的数在该比特位处的比特值.
        let bit = sum % 3;
        ans |= bit << i;
    }
    ans
}

// Bit Manipulation
pub fn single_number3(nums: Vec<i32>) -> i32 {
    let mut ones = 0;
    let mut twos = 0;
    for num in &nums {
        ones ^= num & (!twos);
        twos ^= num & (!ones);
    }

    ones
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![2, 2, 3, 2];
    assert_eq!(func(nums), 3);

    let nums = vec![0, 1, 0, 1, 0, 1, 99];
    assert_eq!(func(nums), 99);
}

fn main() {
    check_solution(single_number1);
    check_solution(single_number2);
    check_solution(single_number3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, single_number1, single_number2, single_number3};

    #[test]
    fn test_single_number1() {
        check_solution(single_number1);
    }

    #[test]
    fn test_single_number2() {
        check_solution(single_number2);
    }

    #[test]
    fn test_single_number3() {
        check_solution(single_number3);
    }
}
