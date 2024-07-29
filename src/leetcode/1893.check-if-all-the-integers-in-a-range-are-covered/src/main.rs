// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashSet;

// Hashset
pub fn is_covered1(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    // 将所有的点位存储到集合中
    let mut set = HashSet::new();
    for range in ranges {
        for i in range[0]..=range[1] {
            set.insert(i);
        }
    }

    // 遍历区间 [left..=right] 上的所有点, 查看它们是否都在集合中
    for i in left..=right {
        if !set.contains(&i) {
            return false;
        }
    }
    true
}

// Merge intervals
pub fn is_covered2(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
    // 依照起始点对区间进行排序
    let mut ranges = ranges;
    ranges.sort_unstable_by_key(|range| range[0]);

    // 合并区间
    let mut intervals = Vec::new();
    let mut start = 0;
    let mut end = 0;
    for range in ranges {
        if range[0] > end + 1 {
            // 区间无法拼接在一起
            if start <= end {
                intervals.push((start, end));
            }
            start = range[0];
            end = range[1];
        } else {
            // 区间可以拼接在一起
            end = end.max(range[1]);
        }
    }
    if start <= end {
        intervals.push((start, end));
    }

    // 查找区间是否有包含
    // 可以使用二分法查找
    for interval in intervals {
        if interval.0 <= left && right <= interval.1 {
            return true;
        }
    }
    false
}

pub type SolutionFn = fn(Vec<Vec<i32>>, i32, i32) -> bool;

fn check_solution(func: SolutionFn) {
    let ranges = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    assert!(func(ranges, 2, 5));

    let ranges = vec![vec![1, 10], vec![10, 20]];
    assert!(!func(ranges, 21, 21));

    let ranges = vec![vec![1, 1]];
    assert!(func(ranges, 1, 1));

    let ranges = vec![vec![50, 50]];
    assert!(!func(ranges, 1, 1));
}

fn main() {
    check_solution(is_covered1);
    check_solution(is_covered2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, is_covered1};

    #[test]
    fn test_is_covered1() {
        check_solution(is_covered1);
    }
}
