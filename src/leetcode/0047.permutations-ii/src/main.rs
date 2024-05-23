// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn permute_unique1(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn backtrack(nums: &[i32], visited: &mut [bool], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        // 终止条件, 数组中所有的元素都访问过了.
        if path.len() == nums.len() {
            res.push(path.clone());
            return;
        }

        // 遍历数组中所有的元素
        for i in 0..nums.len() {
            // 判断是不是重复元素, 重复的元素是否已访问过
            if i > 0 && nums[i] == nums[i - 1] && !visited[i - 1] {
                continue;
            }

            // 判断条件是这个元素还未被访问过.
            if !visited[i] {
                // 访问该元素
                visited[i] = true;
                path.push(nums[i]);

                // 递归搜索
                backtrack(nums, visited, path, res);

                // 撤销访问
                visited[i] = false;
                path.pop();
            }
        }
    }

    let mut nums = nums;
    // 先对数组进行排序, 如果有重复元素的话, 就会紧挨着放.
    nums.sort_unstable();
    // 记录重复的元素是否已经访问过.
    let mut visited = vec![false; nums.len()];

    // 记录所有结果.
    let mut res: Vec<Vec<i32>> = Vec::new();
    // 当前访问路径
    let mut path: Vec<i32> = Vec::new();

    backtrack(&nums, &mut visited, &mut path, &mut res);
    res
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
    let nums = vec![1, 1, 2];
    let expected = vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]];
    assert_eq!(func(nums), expected);

    let nums = vec![1, 2, 3];
    let expected = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];
    assert_eq!(func(nums), expected);
}

fn main() {
    check_solution(permute_unique1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, permute_unique1};

    #[test]
    fn test_permute_unique1() {
        check_solution(permute_unique1);
    }
}
