// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BinaryHeap;

// Priority Queue
pub fn k_closest1(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    assert!(k >= 1 && (k as usize) <= points.len());

    #[derive(Debug, Clone, Eq, PartialEq)]
    struct Entry {
        dist: i64,
        point: Vec<i32>,
    }
    impl PartialOrd for Entry {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Entry {
        fn cmp(&self, other: &Self) -> Ordering {
            self.dist.cmp(&other.dist)
        }
    }

    // 构造优先级队列.
    let k = k as usize;
    let mut heap: BinaryHeap<Entry> = BinaryHeap::with_capacity(k + 1);
    for point in points {
        // 这里, 不需要求平方根, 因为不够精确, 也没必要, 只需要计算出 x*x + y*y 的值即可.
        // 为了处理整数溢出, 先转成 i64.
        let x = point[0] as i64;
        let y = point[1] as i64;
        let dist: i64 = x * x + y * y;
        //println!("dist: {dist}, point: {point:?}");
        heap.push(Entry { dist, point });
        if heap.len() > k {
            heap.pop();
        }
    }

    //    let mut out = Vec::with_capacity(k);
    //    while let Some(top_entry) = heap.pop() {
    //        out.push(top_entry.point);
    //    }
    //    // 再次排序.
    //    out.reverse();
    //    out

    // 导出有序数组.
    heap.into_sorted_vec()
        .into_iter()
        .map(|entry| entry.point)
        .collect()
}

// 有序数组
pub fn k_closest2(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    assert!(k >= 1 && (k as usize) <= points.len());

    // 构造优先级队列.
    let k = k as usize;
    let mut point_dists: Vec<(i64, Vec<i32>)> = points
        .into_iter()
        .map(|point| {
            // 这里, 不需要求平方根, 因为不够精确, 也没必要, 只需要计算出 x*x + y*y 的值即可.
            // 为了处理整数溢出, 先转成 i64.
            let x = point[0] as i64;
            let y = point[1] as i64;
            let dist: i64 = x * x + y * y;
            (dist, point)
        })
        .collect();
    // 根据 dist 升序排列
    point_dists.sort_by(|a, b| a.0.cmp(&b.0));

    // 取前k个元素
    point_dists
        .into_iter()
        .take(k)
        .map(|(_dist, point)| point)
        .collect()
}

// TODO(Shaohua): Quickselect

pub type SolutionFn = fn(Vec<Vec<i32>>, i32) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
    let points = vec![vec![1, 3], vec![-2, 2]];
    let k = 1;
    assert_eq!(func(points, k), vec![vec![-2, 2]]);

    let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
    let k = 1;
    assert_eq!(func(points, k), vec![vec![3, 3]]);

    let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
    let k = 2;
    assert_eq!(func(points, k), vec![vec![3, 3], vec![-2, 4]]);

    let points = vec![vec![9997, 9997], vec![9996, 9998]];
    let k = 1;
    assert_eq!(func(points, k), vec![vec![9997, 9997]]);
}

fn main() {
    check_solution(k_closest1);
    check_solution(k_closest2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, k_closest1, k_closest2};

    #[test]
    fn test_k_closest1() {
        check_solution(k_closest1);
    }

    #[test]
    fn test_k_closest2() {
        check_solution(k_closest2);
    }
}
