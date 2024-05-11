// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// Brute force
//
// 这个方法比较笨, 使用两个数组, 不如直接依次遍历.
// 时间复杂度: O(n^4)
// 空间复杂度: O(n^2)
pub fn four_sum_count1(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    debug_assert!(
        nums1.len() == nums2.len() && nums1.len() == nums3.len() && nums1.len() == nums4.len()
    );
    debug_assert!((0..=200).contains(&nums1.len()));

    // 先计算前两个数组中所有元素的和, 存到新的数组中.
    let mut sums12: Vec<i32> = Vec::new();
    for num1 in &nums1 {
        for num2 in &nums2 {
            sums12.push(num1 + num2);
        }
    }

    // 再计算后两个数组中所有元素的和, 存到新的数组中.
    let mut sums34: Vec<i32> = Vec::with_capacity(nums3.len() * nums4.len());
    for num3 in &nums3 {
        for num4 in &nums4 {
            sums34.push(num3 + num4);
        }
    }

    // 最后遍历数组, 并在集合中查找数值相反的整数.
    let mut count: i32 = 0;
    for sum12 in &sums12 {
        for sum34 in &sums34 {
            if sum12 + sum34 == 0 {
                count += 1;
            }
        }
    }

    count
}

// HashMap
//
// 使用字典计数.
// 时间复杂度: O(n^2)
// 空间复杂度: O(n^2)
pub fn four_sum_count2(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    // 先计算前两个数组中所有元素的和, 存到集合中.
    // 这里 (num, count), 其中 count 表示出现的组合次数.
    let mut map12: HashMap<i32, i32> = HashMap::new();
    for num1 in &nums1 {
        for num2 in &nums2 {
            let sum = num1 + num2;
            *map12.entry(sum).or_default() += 1;
        }
    }

    let mut count: i32 = 0;
    // 再遍历后两个数组.
    for num3 in &nums3 {
        for num4 in &nums4 {
            let sum = num3 + num4;
            if let Some(count12) = map12.get(&(-sum)) {
                count += count12;
            }
        }
    }

    count
}

pub type SolutionFn = fn(Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let nums1 = vec![1, 2];
    let nums2 = vec![-2, -1];
    let nums3 = vec![-1, 2];
    let nums4 = vec![0, 2];
    assert_eq!(func(nums1, nums2, nums3, nums4), 2);

    let nums1 = vec![0];
    let nums2 = vec![0];
    let nums3 = vec![0];
    let nums4 = vec![0];
    assert_eq!(func(nums1, nums2, nums3, nums4), 1);

    let nums1 = vec![-1, -1];
    let nums2 = vec![-1, 1];
    let nums3 = vec![-1, 1];
    let nums4 = vec![1, -1];
    assert_eq!(func(nums1, nums2, nums3, nums4), 6);
}

fn main() {
    check_solution(four_sum_count1);
    check_solution(four_sum_count2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, four_sum_count1, four_sum_count2};

    #[test]
    fn test_four_sum_count1() {
        check_solution(four_sum_count1);
    }

    #[test]
    fn test_four_sum_count2() {
        check_solution(four_sum_count2);
    }
}
