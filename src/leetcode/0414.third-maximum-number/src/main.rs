// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

/// Sort vector and remove duplicates.
/// 对数组排序, 并移除重复元素.
pub fn third_max1(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    nums.dedup();
    if nums.len() < 3 {
        nums[nums.len() - 1]
    } else {
        nums[nums.len() - 3]
    }
}

/// Create an array to hold three maximum values.
/// 创建额外的数组, 用于存储最大的三个数.
pub fn third_max2(nums: Vec<i32>) -> i32 {
    debug_assert!(!nums.is_empty());
    let mut max_nums: [Option<i32>; 3] = [None; 3];
    for num in &nums {
        let mut idx = None;
        for (i, m) in max_nums.iter().enumerate() {
            if let Some(m) = m {
                if num > m {
                    idx = Some(i);
                    break;
                } else if num == m {
                    break;
                }
            } else {
                idx = Some(i);
                break;
            }
        }

        if let Some(idx) = idx {
            for i in (idx + 1..3).rev() {
                max_nums.swap(i, i - 1);
            }
            max_nums[idx] = Some(*num);
        }
    }

    // 如果有第三大的数, 就返回
    if let Some(num) = max_nums[2] {
        num
    } else {
        // 否则返回最大的数
        max_nums[0].unwrap()
    }
}

pub type SolutionFunc = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFunc) {
    let nums = vec![3, 2, 1];
    assert_eq!(func(nums), 1);

    let nums = vec![1, 2];
    assert_eq!(func(nums), 2);

    let nums = vec![2, 2, 3, 1];
    assert_eq!(func(nums), 1);

    let nums = vec![1, 1, 2];
    assert_eq!(func(nums), 2);
}

fn main() {
    check_solution(third_max1);
    check_solution(third_max2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, third_max1, third_max2};

    #[test]
    fn test_third_max1() {
        check_solution(third_max1);
    }

    #[test]
    fn test_third_max2() {
        check_solution(third_max2);
    }
}
