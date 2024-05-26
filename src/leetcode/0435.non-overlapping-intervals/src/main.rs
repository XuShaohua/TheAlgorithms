// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Greedy
pub fn erase_overlap_intervals1(intervals: Vec<Vec<i32>>) -> i32 {
    debug_assert!(!intervals.is_empty());
    // 先按照每个区间的结束值排序.
    let mut intervals = intervals;
    intervals.sort_unstable_by_key(|interval| interval[1]);

    // 统计重叠区间的个数.
    let mut count: i32 = 0;
    // 上个区间的结束值.
    let mut last_end: i32 = i32::MIN;

    // 遍历所有区间.
    for interval in intervals.iter() {
        // 如果上个区间的结束值不大于当前区间的起始值, 说明当前区间与上个区间不重叠.
        if last_end <= interval[0] {
            last_end = interval[1];
        } else {
            // 否则, 这个区间就被忽略掉.
            count += 1;
        }
    }

    count
}

pub type SolutionFn = fn(Vec<Vec<i32>>) -> i32;

fn check_solution(func: SolutionFn) {
    let intervals = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]];
    assert_eq!(func(intervals), 1);

    let intervals = vec![vec![1, 2], vec![1, 2], vec![1, 2]];
    assert_eq!(func(intervals), 2);

    let intervals = vec![vec![1, 2], vec![2, 3]];
    assert_eq!(func(intervals), 0);
}

fn main() {
    check_solution(erase_overlap_intervals1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, erase_overlap_intervals1};

    #[test]
    fn test_erase_overlap_intervals1() {
        check_solution(erase_overlap_intervals1);
    }
}
