// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Sorting
pub fn merge1(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    debug_assert!(intervals.len() >= 2);
    for interval in &intervals {
        debug_assert!(interval.len() == 2);
    }

    let mut intervals = intervals;
    // 基于起始值来排序.
    intervals.sort_by_key(|interval| interval[0]);

    let mut out = Vec::with_capacity(intervals.len());
    let mut start: i32 = intervals[0][0];
    let mut end: i32 = intervals[0][1];

    for mut interval in intervals {
        let (start1, end1) = (interval[0], interval[1]);
        // 完全没有交叉, 保存上一个值, 并更新 (start, end)
        if start1 > end {
            interval[0] = start;
            interval[1] = end;
            out.push(interval);
            start = start1;
            end = end1;
        } else {
            // 有重叠, 只需要更新 end 的值
            end = end.max(end1);
        }
    }

    // 插入最后一组数值
    out.push(vec![start, end]);
    out
}

pub type SolutionFn = fn(Vec<Vec<i32>>) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
    let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
    assert_eq!(func(intervals), expected);

    let intervals = vec![vec![1, 4], vec![4, 5]];
    let expected = vec![vec![1, 5]];
    assert_eq!(func(intervals), expected);
}

fn main() {
    check_solution(merge1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, merge1};

    #[test]
    fn test_merge1() {
        check_solution(merge1);
    }
}
