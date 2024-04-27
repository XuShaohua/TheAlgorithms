// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

// 靠拢型双指针
pub fn max_area1(height: Vec<i32>) -> i32 {
    let len = height.len();
    assert!(len > 1);

    let mut max_area = 0;
    // 两个指针分别从数组的左右两头开始, 往中间靠拢
    let mut left = 0;
    let mut right = len - 1;

    // 循环中止条件是左右两个指针重叠
    while left < right {
        let area: i32;
        if height[left] < height[right] {
            area = (right - left) as i32 * height[left];
            // 一次循环只移动一个指针
            left += 1;
        } else {
            area = (right - left) as i32 * height[right];
            right -= 1;
        }
        // 目标就是找到最大面积
        max_area = area.max(max_area);
    }

    max_area
}

// 优化靠拢型双指针
pub fn max_area2(height: Vec<i32>) -> i32 {
    let len = height.len();
    assert!(len > 1);

    let mut max_area = 0;
    // 两个指针分别从数组的左右两头开始, 往中间靠拢
    let mut left = 0;
    let mut right = len - 1;

    // 循环中止条件是左右两个指针重叠
    while left < right {
        if height[left] < height[right] {
            let curr = height[left];
            let area = (right - left) as i32 * curr;
            // 目标就是找到最大面积
            max_area = area.max(max_area);

            // 内层循环用于跳过无效的高度
            while (left < right) && (height[left] <= curr) {
                left += 1;
            }
        } else {
            let curr = height[right];
            let area = (right - left) as i32 * curr;
            // 目标就是找到最大面积
            max_area = area.max(max_area);

            while (left < right) && (height[right] <= curr) {
                right -= 1;
            }
        }
    }

    max_area
}

pub type SolutionFn = fn(Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let out = func(height);
    assert_eq!(out, 49);

    let height = vec![1, 1];
    let out = func(height);
    assert_eq!(out, 1);
}

fn main() {
    check_solution(max_area1);
    check_solution(max_area2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, max_area1, max_area2};

    #[test]
    fn test_max_area1() {
        check_solution(max_area1);
    }

    #[test]
    fn test_max_area2() {
        check_solution(max_area2);
    }
}
