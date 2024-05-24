// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn combination_sum1(k: i32, n: i32) -> Vec<Vec<i32>> {
    fn backtracking(
        nums: &[i32],
        k: usize,
        target: i32,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        // 终止条件就是 target 不大于 0.
        if target == 0 && path.len() == k {
            res.push(path.clone());
        }
        if target <= 0 {
            return;
        }

        // 遍历所有元素.
        for (index, &num) in nums.iter().enumerate() {
            // 访问元素
            path.push(num);

            // 递归搜索
            backtracking(&nums[index + 1..], k, target - num, path, res);

            // 撤销访问
            path.pop();
        }
    }

    let nums: Vec<i32> = (1..=9).collect();
    // 记录所有结果.
    let mut res: Vec<Vec<i32>> = Vec::new();
    // 记录当前路径.
    let mut path: Vec<i32> = Vec::new();
    let k: usize = k as usize;

    backtracking(&nums, k, n, &mut path, &mut res);
    res
}

pub type SolutionFn = fn(i32, i32) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
    let expected = vec![vec![1, 2, 4]];
    assert_eq!(func(3, 7), expected);

    let expected = vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]];
    assert_eq!(func(3, 9), expected);

    let expected: Vec<Vec<i32>> = vec![];
    assert_eq!(func(4, 1), expected);
}

fn main() {
    check_solution(combination_sum1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, combination_sum1};

    #[test]
    fn test_combination_sum1() {
        check_solution(combination_sum1);
    }
}
