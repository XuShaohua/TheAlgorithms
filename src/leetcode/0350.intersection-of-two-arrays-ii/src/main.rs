// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::collections::HashMap;

// 并行双指针
pub fn intersect1(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    // 先给数组排序
    nums1.sort();
    nums2.sort();

    let mut index1 = 0;
    let mut index2 = 0;
    let len1 = nums1.len();
    let len2 = nums2.len();
    let mut out = Vec::new();

    // 同时遍历两个数组, 只要有一个遍历完成, 就中止.
    while index1 < len1 && index2 < len2 {
        match nums1[index1].cmp(&nums2[index2]) {
            // 两个值不相等, 只移动元素值比较小的那个指针, 另一个指针保持不动.
            Ordering::Less => index1 += 1,
            Ordering::Greater => index2 += 1,
            // 两个元素值相等, 属于交集里的.
            Ordering::Equal => {
                out.push(nums1[index1]);
                // 同时移动两个指针.
                index1 += 1;
                index2 += 1;
                // 这里, 并不需要忽略或者跳过重复元素, 因为它们也是有效的.
            }
        }
    }
    out
}

// 用哈稀表计数
pub fn intersect2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // 用哈稀表存储有较少元素的数组.
    let (nums1, nums2) = if nums1.len() < nums2.len() {
        (nums1, nums2)
    } else {
        (nums2, nums1)
    };

    // 用哈稀表来给 nums1 中的元素计数.
    let mut map1 = HashMap::new();
    for num in &nums1 {
        map1.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut out = Vec::new();

    // 遍历 nums2.
    for num in &nums2 {
        if let Some(count) = map1.get_mut(num) {
            // 如果该元素在 map1 中存在, 就将其计数值减1.
            *count -= 1;
            out.push(*num);
            if *count == 0 {
                map1.remove(num);
            }
        }
    }
    out
}

pub type SolutionFn = fn(Vec<i32>, Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    #[inline]
    fn nearly_eq(mut nums1: Vec<i32>, mut nums2: Vec<i32>) {
        nums1.sort();
        nums2.sort();
        assert_eq!(nums1, nums2);
    }

    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    nearly_eq(func(nums1, nums2), vec![2, 2]);

    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    nearly_eq(func(nums1, nums2), vec![4, 9]);
}

fn main() {
    check_solution(intersect1);
    check_solution(intersect2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, intersect1, intersect2};

    #[test]
    fn test_intersect1() {
        check_solution(intersect1);
    }

    #[test]
    fn test_intersect2() {
        check_solution(intersect2);
    }
}
