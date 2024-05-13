// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::HashMap;

// HashTable
pub fn number_of_boomerangs1(points: Vec<Vec<i32>>) -> i32 {
    #[allow(unused)]
    fn factorial(n: usize) -> usize {
        (1..=n).product()
    }
    #[allow(unused)]
    fn combination(n: usize) -> usize {
        if n == 1 {
            1
        } else {
            n * (n - 1)
        }
    }

    debug_assert!(!points.is_empty());
    let len = points.len();
    if len < 3 {
        return 0;
    }

    // ((起始点, 两点间的距离的平方), 计数)
    let mut map: HashMap<(i32, i32, i32), usize> = HashMap::new();

    // 遍历所有的点, 计算它们间的距离.
    for i in 0..len {
        for j in 0..len {
            if i == j {
                continue;
            }

            let p1: &[i32] = &points[i];
            let p2: &[i32] = &points[j];
            let distance: i32 = (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2);
            // 加入字典中.
            let key = (p1[0], p1[1], distance);
            *map.entry(key).or_default() += 1;
        }
    }

    // 统计所有的元组
    let mut total_count: usize = 0;
    for &count in map.values() {
        if count > 1 {
            // 使用组合数.
            total_count += combination(count);
        }
    }
    total_count as i32
}

pub type SolutionFn = fn(Vec<Vec<i32>>) -> i32;

fn check_solution(func: SolutionFn) {
    let points = vec![vec![0, 0], vec![1, 0], vec![2, 0]];
    assert_eq!(func(points), 2);

    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    assert_eq!(func(points), 2);

    let points = vec![vec![1, 1]];
    assert_eq!(func(points), 0);

    let points = vec![vec![0, 0], vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]];
    assert_eq!(func(points), 20);
}

fn main() {
    check_solution(number_of_boomerangs1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, number_of_boomerangs1};

    #[test]
    fn test_number_of_boomerangs1() {
        check_solution(number_of_boomerangs1);
    }
}
