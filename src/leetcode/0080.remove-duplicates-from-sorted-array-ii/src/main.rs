// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 快慢型双指针
pub fn remove_duplicates1(nums: &mut Vec<i32>) -> i32 {
    assert!(!nums.is_empty());
    let len = nums.len();

    // 慢指针指向结果数组的最高位
    let mut slow = 0;
    // 用快指针遍历数组
    let mut fast = 0;

    // 遍历数组
    while fast < len {
        let curr = nums[fast];
        // 元素重复的次数
        let mut dup = 0;

        // 复制前两个重复的元素, 而忽略后面的.
        while (fast < len) && (curr == nums[fast]) {
            if dup < 2 {
                nums[slow] = curr;
                // 同时移动慢指针, 结果数组最高位 +1
                slow += 1;
            }
            dup += 1;
            fast += 1;
        }
    }

    slow as i32
}

// 优化双指针
pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
    assert!(!nums.is_empty());
    let len = nums.len();
    if len < 3 {
        return len as i32;
    }

    // 慢指针指向结果数组的最高位, 跳过前两个元素
    let mut slow = 2;

    // 用快指针遍历数组, 跳过前两个元素
    for fast in 2..len {
        // 这里是关键点!
        // `slow - 2` 表示允许有两个重复的元素.
        if nums[fast] != nums[slow - 2] {
            // 移动慢指针
            nums[slow] = nums[fast];
            slow += 1;
        }
    }

    slow as i32
}

pub type SolutionFn = fn(&mut Vec<i32>) -> i32;

fn check_solution(func: SolutionFn) {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let k = func(&mut nums) as usize;
    assert_eq!(k, 5);
    assert_eq!(&nums[..k], &[1, 1, 2, 2, 3]);

    let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    let k = func(&mut nums) as usize;
    assert_eq!(k, 7);
    assert_eq!(&nums[..k], &[0, 0, 1, 1, 2, 3, 3]);
}

fn main() {
    check_solution(remove_duplicates1);
    check_solution(remove_duplicates2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, remove_duplicates1, remove_duplicates2};

    #[test]
    fn test_remove_duplicates1() {
        check_solution(remove_duplicates1);
    }

    #[test]
    fn test_remove_duplicates2() {
        check_solution(remove_duplicates2);
    }
}
