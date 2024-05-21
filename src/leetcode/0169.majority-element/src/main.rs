// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Sorting
// 如果该数出现次数过半, 那么当数组排序后, 位于数组中间的那个数, 一定就是它.
pub fn majority_element1(nums: Vec<i32>) -> i32 {
    debug_assert!(!nums.is_empty());
    let mut nums = nums;
    nums.sort_unstable();
    nums[nums.len() / 2]
}

// 摩尔投票法
pub fn majority_element2(nums: Vec<i32>) -> i32 {
    let mut major: i32 = 0;
    let mut count: usize = 0;
    for &num in &nums {
        if count == 0 {
            // 如果 count 为 0, 把当前数作为 major
            major = num;
        } else if num == major {
            // 如果当前整数与 major 相等, 计数就加 1
            count += 1;
        } else {
            // 如果当前整数与 major 不相等, 计数减 1
            count -= 1;
        }
    }

    // 检验是否真的超过一半
    let mut check_count: usize = 0;
    for &num in &nums {
        if num == major {
            check_count += 1;
        }
    }

    if check_count * 2 > nums.len() {
        major
    } else {
        // 如果没有超过一半, 就返回 -1
        -1
    }
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![3, 2, 3];
    assert_eq!(func(nums), 3);

    let nums = vec![2, 2, 1, 1, 1, 2, 2];
    assert_eq!(func(nums), 2);
}

fn main() {
    check_solution(majority_element1);
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
