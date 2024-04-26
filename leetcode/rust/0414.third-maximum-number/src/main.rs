// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub type SolutionFunc = fn(Vec<i32>) -> i32;

/// Sort vector and remove duplicates.
pub fn third_max(nums: Vec<i32>) -> i32 {
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
pub fn third_max2(nums: Vec<i32>) -> i32 {
    let mut max_nums: [Option<i32>; 3] = [None; 3];
    for num in nums {
        let mut idx = None;
        for (i, m) in max_nums.iter().enumerate() {
            if m.is_some() && num == m.unwrap() {
                break;
            }

            if m.is_none() || m.is_some() && num > m.unwrap() {
                idx = Some(i);
                break;
            }
        }

        if let Some(idx) = idx {
            for i in (idx + 1..3).rev() {
                max_nums.swap(i, i - 1);
            }
            max_nums[idx] = Some(num);
        }
    }

    if let Some(num) = max_nums[2] {
        num
    } else {
        max_nums[0].unwrap()
    }
}

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
    check_solution(third_max);
    check_solution(third_max2);
}
