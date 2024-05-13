// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::{HashMap, HashSet};

// Bruteforce
// 字典计数
pub fn max_points1(points: Vec<Vec<i32>>) -> i32 {
    // 返回 y = ax+b 的 (a, b) 信息
    // 如果这两个点的 x 值相等, 那就无法组成 x-y 平面上的直线,
    // 我们单独处理它.
    fn get_line(p1: &[i32], p2: &[i32]) -> (f32, f32) {
        assert!(p1.len() == p2.len() && p1.len() == 2);
        let x1: i32 = p1[0];
        let y1: i32 = p1[1];
        let x2: i32 = p2[0];
        let y2: i32 = p2[1];
        if x1 == x2 {
            return (x1 as f32, f32::MAX);
        }

        let a: f32 = (y1 - y2) as f32 / (x1 - x2) as f32;
        let b: f32 = if x1 != 0 {
            y1 as f32 - a * x1 as f32
        } else {
            y2 as f32 - a * x2 as f32
        };
        (a, b)
    }

    debug_assert!(!points.is_empty());
    let len: usize = points.len();
    if len == 1 {
        return 1;
    }

    let mut map: HashMap<(String, String), HashSet<(i32, i32)>> = HashMap::with_capacity(len * len);

    // 嵌套遍历两次数组, 计算两点之间形成的直线, 并将直线信息存放字典.
    // 直线是 y = ax+b, 以 (a, b) 元组作为字典的键.
    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            let p1 = &points[i];
            let p2 = &points[j];
            let line = get_line(p1, p2);
            let line_str = (line.0.to_string(), line.1.to_string());
            let set = map.entry(line_str).or_default();
            set.insert((p1[0], p1[1]));
            set.insert((p2[0], p2[1]));
        }
    }
    //println!("map: {map:?}");

    let mut max_points: usize = 0;
    for set in map.values() {
        max_points = max_points.max(set.len());
    }
    max_points as i32
}

// HashTable
// 字典计数
// 但每次只考虑第一个点作为起始点, 简化直线的构造;
// 同时, 用 i64 表示斜率, 而不是用字符串.
pub fn max_points2(points: Vec<Vec<i32>>) -> i32 {
    debug_assert!(!points.is_empty());

    let len: usize = points.len();
    if len == 1 {
        return 1;
    }

    let mut max_points: usize = 0;
    let mut lines: HashMap<i64, usize> = HashMap::with_capacity(len);
    // 将 i32 转为 i64, 左移32位, 以保留较好的精度.
    const SHIFT_LEFT: i64 = 32;

    // 第一层循环, 确定直线的第一个点.
    for i in 0..(len - 1) {
        // 第二层循环, 确定直线的第二个点.
        for j in (i + 1)..len {
            // 我们只需要确认 y = ax + b 中 a 的值即可, 因为这些直线都通过第一个点.

            let p1 = &points[i];
            let p2 = &points[j];
            let a: i64 = if p1[0] == p2[0] {
                // 处理特殊情况, x1 == x2
                i64::MAX
            } else {
                // (y1 - y2) / (x1 - x2)
                (((p1[1] - p2[1]) as i64) << SHIFT_LEFT) / (p1[0] - p2[0]) as i64
            };

            // 如果这条直线不存在, 就加入两个点;
            // 否则只加入第二个点.
            lines.entry(a).and_modify(|count| *count += 1).or_insert(2);
        }

        // 求最大值
        let points: usize = lines.values().max().copied().unwrap_or_default();
        max_points = max_points.max(points);
        lines.clear();
    }

    max_points as i32
}

pub type SolutionFn = fn(Vec<Vec<i32>>) -> i32;

fn check_solution(func: SolutionFn) {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    assert_eq!(func(points), 3);

    let points = vec![
        vec![1, 1],
        vec![3, 2],
        vec![5, 3],
        vec![4, 1],
        vec![2, 3],
        vec![1, 4],
    ];
    assert_eq!(func(points), 4);

    let points = vec![vec![0, 0]];
    assert_eq!(func(points), 1);

    let points = vec![vec![1, 0], vec![0, 0]];
    assert_eq!(func(points), 2);

    let points = vec![vec![0, 1], vec![0, 0]];
    assert_eq!(func(points), 2);

    let points = vec![
        vec![0, 0],
        vec![1, 1],
        vec![3, 4],
        vec![4, 5],
        vec![5, 6],
        vec![7, 8],
        vec![8, 9],
    ];
    assert_eq!(func(points), 5);
}

fn main() {
    check_solution(max_points1);
    check_solution(max_points2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, max_points1, max_points2};

    #[test]
    fn test_max_points1() {
        check_solution(max_points1);
    }

    #[test]
    fn test_max_points2() {
        check_solution(max_points2);
    }
}
