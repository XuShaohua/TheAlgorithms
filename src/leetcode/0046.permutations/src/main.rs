// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// Backtracking
pub fn permute1(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn inner(nums: &[i32], path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if nums.len() == path.len() {
            res.push(path.clone());
            return;
        }

        for &num in nums.iter() {
            if !path.contains(&num) {
                path.push(num);
                inner(nums, path, res);
                path.pop();
            }
        }
    }

    let mut path = Vec::new();
    let mut res = Vec::new();
    inner(&nums, &mut path, &mut res);
    res
}

pub type SolutionFn = fn(Vec<i32>) -> Vec<Vec<i32>>;

fn check_solution(func: SolutionFn) {
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

    let nums = vec![0, 1];
    let expected = vec![vec![0, 1], vec![1, 0]];
    assert_eq!(func(nums), expected);
    let nums = vec![1];
    let expected = vec![vec![1]];
    assert_eq!(func(nums), expected);
}

fn main() {
    check_solution(permute1);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, permute1};

    #[test]
    fn test_permute1() {
        check_solution(permute1);
    }
}
