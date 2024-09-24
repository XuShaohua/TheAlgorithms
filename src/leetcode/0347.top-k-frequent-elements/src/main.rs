// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::{BinaryHeap, HashMap};

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

    // 优先队列, BinaryHeap 是一个最大堆
    let k = k as usize;
    let mut heap = BinaryHeap::with_capacity(map.len());
    for (num, count) in map {
        heap.push((count, num));
    }

    // 转换成数组.
    let mut out = Vec::with_capacity(k);
    while let Some(top) = heap.pop() {
        out.push(top.1);
        if out.len() == k {
            break;
        }
    }
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

// HashMap + Bucket
pub fn top_k_frequent3(nums: Vec<i32>, k: i32) -> Vec<i32> {
    assert!(!nums.is_empty());
    assert!(k > 0);

    // 计数
    let mut count_map: HashMap<i32, usize> = HashMap::new();
    for &num in &nums {
        *count_map.entry(num).or_insert(0) += 1;
    }

    // 将有相同频率的数值放在一个数组中.
    let max_count: usize = count_map.values().max().copied().unwrap_or_default();
    // 要注意数组的元素个数是 max_count + 1
    let mut count_list = vec![Vec::new(); max_count + 1];
    for (&num, &count) in &count_map {
        count_list[count].push(num);
    }

    // 从最高频率开始, 依次收集整数值.
    let k_usize = k as usize;
    let mut out = Vec::new();
    for array in count_list.into_iter().rev() {
        if !array.is_empty() {
            out.extend(&array);
        }
        if out.len() >= k_usize {
            break;
        }
    }
    out
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
