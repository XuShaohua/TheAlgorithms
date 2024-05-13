// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::{self, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
struct Entry {
    num: i32,
    count: Reverse<usize>,
}

impl cmp::PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

// HashMap + Priority Queue
// 字典计数
pub fn top_k_frequent1(nums: Vec<i32>, k: i32) -> Vec<i32> {
    assert!(!nums.is_empty());
    assert!(k > 0);

    // 计数
    let mut map: HashMap<i32, usize> = HashMap::new();
    for &num in &nums {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    // 优先队列
    let k = k as usize;
    let mut heap: BinaryHeap<Entry> = BinaryHeap::with_capacity(k + 1);
    for (num, count) in map {
        heap.push(Entry {
            count: Reverse(count),
            num,
        });
        if heap.len() > k {
            heap.pop();
        }
    }

    // 转换成数组.
    let mut out = Vec::new();
    while let Some(top) = heap.pop() {
        out.push(top.num);
    }
    out.reverse();
    out
}

// HashMap + 数组
// 字典计数
pub fn top_k_frequent2(nums: Vec<i32>, k: i32) -> Vec<i32> {
    assert!(!nums.is_empty());
    assert!(k > 0);

    // 计数
    let mut map: HashMap<i32, usize> = HashMap::new();
    for &num in &nums {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    // 有序数组.
    let k = k as usize;
    let mut num_counts: Vec<(i32, usize)> = map.into_iter().collect();
    num_counts.sort_unstable_by_key(|(_num, count)| *count);

    // 转换成整数数组.
    num_counts[num_counts.len() - k..]
        .iter()
        .rev()
        .map(|(num, _count)| *num)
        .collect()
}

// HashMap + 数组
// 字典计数
// 优化数组
pub fn top_k_frequent3(nums: Vec<i32>, k: i32) -> Vec<i32> {
    assert!(!nums.is_empty());
    assert!(k > 0);

    // 计数
    let mut map: HashMap<i32, usize> = HashMap::new();
    for &num in &nums {
        map.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    // 有序数组.
    let k = k as usize;
    let mut num_counts: Vec<(i32, usize)> = map.into_iter().collect();
    // 降序排列
    num_counts.sort_by(|a, b| b.1.cmp(&a.1));

    // 转换成整数数组.
    num_counts
        .into_iter()
        .take(k)
        .map(|(num, _count)| num)
        .collect()
}

pub type SolutionFn = fn(Vec<i32>, i32) -> Vec<i32>;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 1, 1, 1, 2, 2, 3, 4];
    let k = 2;
    assert_eq!(func(nums, k), vec![1, 2]);

    let nums = vec![1];
    let k = 1;
    assert_eq!(func(nums, k), vec![1]);
}

fn main() {
    check_solution(top_k_frequent1);
    check_solution(top_k_frequent2);
    check_solution(top_k_frequent3);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, top_k_frequent1, top_k_frequent2, top_k_frequent3};

    #[test]
    fn test_top_k_frequent1() {
        check_solution(top_k_frequent1);
    }

    #[test]
    fn test_top_k_frequent2() {
        check_solution(top_k_frequent2);
    }

    #[test]
    fn test_top_k_frequent3() {
        check_solution(top_k_frequent3);
    }
}