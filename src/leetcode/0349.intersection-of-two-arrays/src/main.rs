// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::Ordering;
use std::collections::HashSet;

// 并行双指针
pub fn intersection1(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut nums1 = nums1;
    let mut nums2 = nums2;
    // 先给数组排序
    nums1.sort();
    nums2.sort();

    let mut index1 = 0;
    let mut index2 = 0;
    let len1 = nums1.len();
    let len2 = nums2.len();
    let mut out = Vec::with_capacity(len1 + len2);

    // 遍历两个数组
    while index1 < len1 && index2 < len2 {
        match nums1[index1].cmp(&nums2[index2]) {
            Ordering::Less => {
                index1 += 1;
            }
            Ordering::Equal => {
                let val = nums1[index1];
                out.push(val);
                // 跳过重复的元素
                while index1 < len1 && nums1[index1] == val {
                    index1 += 1;
                }
                while index2 < len2 && nums2[index2] == val {
                    index2 += 1;
                }
            }
            Ordering::Greater => {
                index2 += 1;
            }
        }
    }
    out
}

// 使用 HashSet 集合操作
pub fn intersection2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1: HashSet<i32> = nums1.into_iter().collect();
    let set2: HashSet<i32> = nums2.into_iter().collect();
    set1.intersection(&set2).copied().collect()
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct BitSet {
    bits: Vec<bool>,
}

impl BitSet {
    #[must_use]
    #[inline]
    pub const fn new() -> Self {
        Self { bits: Vec::new() }
    }

    #[must_use]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bits: Vec::with_capacity(capacity),
        }
    }

    #[inline]
    pub fn set(&mut self, index: usize) {
        if index >= self.bits.len() {
            self.bits.resize(index + 1, false);
        }
        self.bits[index] = true;
    }

    #[inline]
    pub fn unset(&mut self, index: usize) {
        if index < self.bits.len() {
            self.bits[index] = false;
        }
    }

    #[must_use]
    #[inline]
    pub fn is_set(&self, index: usize) -> bool {
        if index < self.bits.len() {
            self.bits[index]
        } else {
            false
        }
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<usize> {
        // TODO(shaohua): Impl Iterator and IntoIter traits
        self.bits
            .iter()
            .enumerate()
            .filter(|(_index, &is_set)| is_set)
            .map(|(index, _is_set)| index)
            .collect()
    }
}

impl FromIterator<usize> for BitSet {
    fn from_iter<T>(iter: T) -> Self
        where
            T: IntoIterator<Item=usize>,
    {
        let iterator = iter.into_iter();
        let capacity = match iterator.size_hint() {
            (_, Some(upper_size)) => upper_size,
            (size, None) => size,
        };
        let mut set = BitSet::with_capacity(capacity);
        for num in iterator {
            set.set(num)
        }
        set
    }
}

// 使用 BitSet
pub fn intersection3(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1: BitSet = nums1.iter().map(|&val| val as usize).collect();
    let mut set2 = BitSet::with_capacity(nums2.len());
    let mut out = Vec::new();
    for &num in &nums2 {
        let num_usize = num as usize;
        // num 只在 set1 中存在, 在 set2 中不存在
        if set1.is_set(num_usize) && !set2.is_set(num_usize) {
            out.push(num);
        }
        set2.set(num_usize);
    }
    out
}

// 优化上面的方法, 只使用一个 bitset
pub fn intersection4(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    // 将 nums1 转换为 bitset.
    let mut set1: BitSet = nums1.iter().map(|&val| val as usize).collect();
    let mut out = Vec::new();

    // 遍历 nums2
    for &num in &nums2 {
        let num_usize = num as usize;
        // num 在 set1 中也存在
        if set1.is_set(num_usize) {
            out.push(num);
            // 重置 set1 中的值, 因为它已经被插入到了结果中, 不能再重复使用.
            set1.unset(num_usize);
        }
    }
    out
}

pub type SolutionFn = fn(Vec<i32>, Vec<i32>) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    fn nearly_equal(mut vec1: Vec<i32>, mut vec2: Vec<i32>) {
        vec1.sort();
        vec2.sort();
        assert_eq!(vec1, vec2);
    }
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    nearly_equal(func(nums1, nums2), vec![2]);

    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    nearly_equal(func(nums1, nums2), vec![9, 4]);
}

fn main() {
    check_solution(intersection1);
    check_solution(intersection2);
    check_solution(intersection3);
    check_solution(intersection4);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, intersection1, intersection2, intersection3, intersection4};

    #[test]
    fn test_intersection1() {
        check_solution(intersection1);
    }

    #[test]
    fn test_intersection2() {
        check_solution(intersection2);
    }

    #[test]
    fn test_intersection3() {
        check_solution(intersection3);
    }

    #[test]
    fn test_intersection4() {
        check_solution(intersection4);
    }
}
