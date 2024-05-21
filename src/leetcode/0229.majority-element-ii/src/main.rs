// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// HashTable
// 字典计数
pub fn majority_element1(nums: Vec<i32>) -> Vec<i32> {
    debug_assert!(!nums.is_empty());

    let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());
    let mut out = Vec::new();

    for &num in &nums {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    let len: usize = nums.len();
    for (num, count) in map {
        if count * 3 > len {
            out.push(num);
        }
    }

    out
}

// 摩尔投票法
// 进行变形, 创建两个 major, 分别代表其中的 1/3 份额.
pub fn majority_element2(nums: Vec<i32>) -> Vec<i32> {
    debug_assert!(!nums.is_empty());

    let mut major1: i32 = i32::MIN;
    let mut major2: i32 = i32::MIN;
    let mut count1: usize = 0;
    let mut count2: usize = 0;

    for &num in &nums {
        if major1 == num {
            count1 += 1;
        } else if major2 == num {
            count2 += 1;
        } else if count1 == 0 {
            major1 = num;
            count1 = 1;
        } else if count2 == 0 {
            major2 = num;
            count2 = 1;
        } else {
            count1 -= 1;
            count2 -= 1;
        }
    }

    // 检验是否真的超过 1/3
    let mut check_count1: usize = 0;
    let mut check_count2: usize = 0;
    for &num in &nums {
        if num == major1 {
            check_count1 += 1;
        } else if num == major2 {
            check_count2 += 1;
        }
    }

    let mut out = Vec::new();
    if check_count1 * 3 > nums.len() {
        out.push(major1);
    }
    if check_count2 * 3 > nums.len() {
        out.push(major2);
    }
    out.sort_unstable();
    out
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let nums = vec![3, 2, 3];
    assert_eq!(func(nums), vec![3]);

    let nums = vec![1];
    assert_eq!(func(nums), vec![1]);

    let nums = vec![1, 2];
    assert_eq!(func(nums), vec![1, 2]);

    let nums = vec![2, 1, 1, 3, 1, 4, 5, 6];
    assert_eq!(func(nums), vec![1]);
}

fn main() {
    //check_solution(majority_element1);
    check_solution(majority_element2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, majority_element1, majority_element2};

    #[test]
    fn test_majority_element1() {
        check_solution(majority_element1);
    }

    #[test]
    fn test_majority_element2() {
        check_solution(majority_element2);
    }
}
