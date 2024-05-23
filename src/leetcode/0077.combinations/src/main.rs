// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Backtracking
pub fn combine1(n: i32, k: i32) -> Vec<Vec<i32>> {
    fn backtrack(
        nums: &[i32],
        index: usize,
        k: usize,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        // 终止条件, path 中包含了 k 个元素.
        if path.len() == k {
            res.push(path.clone());
            return;
        }

        // 遍历数组中所有未访问过的元素
        for i in index..nums.len() {
            // 访问元素
            path.push(nums[i]);

            // 递归搜索
            backtrack(nums, i + 1, k, path, res);

            // 撤销访问
            path.pop();
        }
    }

    debug_assert!(k >= 1 && n >= k);
    // 构造数组.
    let nums: Vec<i32> = (1..=n).collect();
    let k: usize = k as usize;
    // 存放所有结果.
    let mut res: Vec<Vec<i32>> = Vec::new();
    // 存放当前路径上的结果.
    let mut path: Vec<i32> = Vec::new();
    backtrack(&nums, 0, k, &mut path, &mut res);
    res
}

pub type SolutionFn = fn(i32, i32) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
    let n = 4;
    let k = 2;
    let expected = vec![
        vec![1, 2],
        vec![1, 3],
        vec![1, 4],
        vec![2, 3],
        vec![2, 4],
        vec![3, 4],
    ];
    assert_eq!(func(n, k), expected);

    let n = 1;
    let k = 1;
    let expected = vec![vec![1]];
    assert_eq!(func(n, k), expected);
}

fn main() {
    check_solution(combine1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, combine1};

    #[test]
    fn test_combine1() {
        check_solution(combine1);
    }
}
