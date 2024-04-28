// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

// Brute force
// 直接调用数组的方法
#[allow(clippy::ptr_arg)]
pub fn merge1(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let m = m as usize;
    let n = n as usize;
    assert_eq!(nums2.len(), n);
    nums1[m..m + n].copy_from_slice(nums2);
    // 合并两个数组, 然后排序
    nums1.sort();
}

// 并行双指针
#[allow(clippy::ptr_arg)]
pub fn merge2(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    // 两个指针分别指向两个数组的尾部元素.
    let mut index1 = m - 1;
    let mut index2 = n - 1;
    // 这个指针指当前向合并后的数组的最低位.
    let mut new_index = m + n - 1;

    // 从数组的尾部向头部合并, 即从高位开始, 直到有一个数组合并完成中止.
    while index1 >= 0 && index2 >= 0 {
        let index1_usize = index1 as usize;
        let index2_usize = index2 as usize;
        let new_index_usize = new_index as usize;
        if nums1[index1_usize] > nums2[index2_usize] {
            nums1[new_index_usize] = nums1[index1_usize];
            index1 -= 1;
        } else {
            nums1[new_index_usize] = nums2[index2_usize];
            index2 -= 1;
        }
        new_index -= 1;
    }

    // 如果 nums2 还没有合并完, 就把剩下的元素都合并过来
    while index2 >= 0 {
        nums1[new_index as usize] = nums2[index2 as usize];
        index2 -= 1;
        new_index -= 1;
    }
}

pub type SolutionFn = fn(&mut Vec<i32>, m: i32, &mut Vec<i32>, n: i32);

fn check_solution(func: SolutionFn) {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;
    let expected_out = vec![1, 2, 2, 3, 5, 6];
    func(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![1];
    let m = 1;
    let mut nums2 = vec![];
    let n = 0;
    let expected_out = vec![1];
    func(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![0];
    let m = 0;
    let mut nums2 = vec![1];
    let n = 1;
    let expected_out = vec![1];
    func(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![21, 22, 23, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;
    let expected_out = vec![2, 5, 6, 21, 22, 23];
    func(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![21, 22, 23, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![32, 35, 36];
    let n = 3;
    let expected_out = vec![21, 22, 23, 32, 35, 36];
    func(&mut nums1, m, &mut nums2, n);
    assert_eq!(nums1, expected_out);
}

fn main() {
    check_solution(merge1);
    check_solution(merge2);
}

#[cfg(test)]
mod tests {
    use super::{check_solution, merge1, merge2};

    #[test]
    fn test_merge1() {
        check_solution(merge1);
    }

    #[test]
    fn test_merge2() {
        check_solution(merge2);
    }
}
