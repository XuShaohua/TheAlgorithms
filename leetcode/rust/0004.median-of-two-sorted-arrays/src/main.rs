// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let len = nums1.len() + nums2.len();
    let mut merged = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = 0;

    while i < nums1.len() && j < nums2.len() {
        if nums1[i] <= nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }
    while i < nums1.len() {
        merged.push(nums1[i]);
        i += 1;
    }
    while j < nums2.len() {
        merged.push(nums2[j]);
        j += 1;
    }

    let half = len / 2;
    if len % 2 == 0 {
        (merged[half - 1] + merged[half]) as f64 / 2.0
    } else {
        merged[half] as f64
    }
}

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let median = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(median, 2.0);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    let median = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(median, 2.5);
}
