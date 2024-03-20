// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

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

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let nums2 = vec![2, 5, 6];
    let n = 3;
    let expected_out = vec![1, 2, 2, 3, 5, 6];
    merge(&mut nums1, m, &nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![1];
    let m = 1;
    let nums2 = vec![];
    let n = 0;
    let expected_out = vec![1];
    merge(&mut nums1, m, &nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![0];
    let m = 0;
    let nums2 = vec![1];
    let n = 1;
    let expected_out = vec![1];
    merge(&mut nums1, m, &nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![21, 22, 23, 0, 0, 0];
    let m = 3;
    let nums2 = vec![2, 5, 6];
    let n = 3;
    let expected_out = vec![2, 5, 6, 21, 22, 23];
    merge(&mut nums1, m, &nums2, n);
    assert_eq!(nums1, expected_out);

    let mut nums1 = vec![21, 22, 23, 0, 0, 0];
    let m = 3;
    let nums2 = vec![32, 35, 36];
    let n = 3;
    let expected_out = vec![21, 22, 23, 32, 35, 36];
    merge(&mut nums1, m, &nums2, n);
    assert_eq!(nums1, expected_out);
}
