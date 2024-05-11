// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 排序
// 时间复杂度: O(nlogn)
// 空间复杂度: O(1)
#[allow(clippy::comparison_chain)]
pub fn first_missing_positive1(nums: Vec<i32>) -> i32 {
    debug_assert!(!nums.is_empty());
    let mut nums = nums;
    nums.sort_unstable();

    // 记录最小的整数.
    let mut min: i32 = 1;

    // 遍历数组.
    for &num in &nums {
        if num == min {
            min += 1;
        } else if num > min {
            // 如果当前整数比最小整数大, 那就可以直接返回.
            return min;
        }
    }
    min
}

// In-place HashTable
// TODO(Shaohua): From @Mikazuki4712
pub fn first_missing_positive2(nums: Vec<i32>) -> i32 {
    assert!(!nums.is_empty());
    let mut nums = nums;
    let len: usize = nums.len();
    let len_i32: i32 = len as i32;

    for num in nums.iter_mut() {
        if *num <= 0 || *num > len_i32 {
            *num = len_i32 + 1;
        }
    }
    for i in 0..len {
        let val: i32 = nums[i].abs();
        if val >= 1 && val <= len_i32 {
            let flag_index: usize = val as usize - 1;
            if nums[flag_index] > 0 {
                nums[flag_index] *= -1;
            }
        }
    }

    for i in 1..=len {
        if nums[i - 1] > 0 {
            return i as i32;
        }
    }

    len_i32 + 1
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 2, 0];
    assert_eq!(func(nums), 3);

    let nums = vec![3, 4, -1, 1];
    assert_eq!(func(nums), 2);

    let nums = vec![7, 8, 9, 11, 12];
    assert_eq!(func(nums), 1);
}

fn main() {
    check_solution(first_missing_positive1);
    check_solution(first_missing_positive2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, first_missing_positive1, first_missing_positive2};

    #[test]
    fn test_first_missing_positive1() {
        check_solution(first_missing_positive1);
    }

    #[test]
    fn test_first_missing_positive2() {
        check_solution(first_missing_positive2);
    }
}
