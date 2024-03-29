// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub type MergeFunc = fn(&mut [i32], i32, &[i32], i32);

pub fn merge(nums1: &mut [i32], m: i32, nums2: &[i32], n: i32) {
    let m = m as usize;
    let n = n as usize;
    assert_eq!(nums1.len(), m + n);

    if m == 0 {
        for (index, num) in nums2.iter().enumerate() {
            nums1[index] = *num;
        }
        return;
    }
    if n == 0 {
        return;
    }

    // source indexes.
    let mut i = m - 1;
    let mut j = n - 1;
    // target index
    let mut k = m + n - 1;

    loop {
        if nums1[i] >= nums2[j] {
            nums1[k] = nums1[i];
            if i == 0 {
                break;
            }
            i -= 1;
        } else {
            nums1[k] = nums2[j];
            if j == 0 {
                return;
            }
            j -= 1;
        }
        k -= 1;
    }

    loop {
        nums1[j] = nums2[j];
        if j == 0 {
            break;
        }
        j -= 1;
    }
}

/// Two pointers
pub fn merge2(nums1: &mut [i32], m: i32, nums2: &[i32], n: i32) {
    let mut i = m - 1;
    let mut new_index = nums1.len() - 1;
    let mut j = n - 1;

    while i >= 0 && j >= 0 {
        if nums1[i as usize] > nums2[j as usize] {
            nums1[new_index] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[new_index] = nums2[j as usize];
            j -= 1;
        }
        new_index -= 1;
    }

    while j >= 0 {
        nums1[new_index] = nums2[j as usize];
        j -= 1;
        if new_index > 0 {
            new_index -= 1;
        }
    }
}

fn check_solution(func: MergeFunc) {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let nums2 = vec![2, 5, 6];
    let n = 3;
    let expected_out = vec![1, 2, 2, 3, 5, 6];
    func(&mut nums1, m, &nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![1];
    let m = 1;
    let nums2 = vec![];
    let n = 0;
    let expected_out = vec![1];
    func(&mut nums1, m, &nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![0];
    let m = 0;
    let nums2 = vec![1];
    let n = 1;
    let expected_out = vec![1];
    func(&mut nums1, m, &nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![21, 22, 23, 0, 0, 0];
    let m = 3;
    let nums2 = vec![2, 5, 6];
    let n = 3;
    let expected_out = vec![2, 5, 6, 21, 22, 23];
    func(&mut nums1, m, &nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![21, 22, 23, 0, 0, 0];
    let m = 3;
    let nums2 = vec![32, 35, 36];
    let n = 3;
    let expected_out = vec![21, 22, 23, 32, 35, 36];
    func(&mut nums1, m, &nums2, n);
    assert_eq!(nums1, expected_out);
}

fn main() {
    //check_solution(merge);
    check_solution(merge2);
}
