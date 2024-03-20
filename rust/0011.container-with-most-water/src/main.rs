// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn max_area(height: Vec<i32>) -> i32 {
    let len = height.len();
    assert!(len > 1);

    let mut max_area = 0;
    let mut left = 0;
    let mut right = len - 1;

    while left < right {
        let area: i32;
        if height[left] < height[right] {
            area = (right - left) as i32 * height[left];
            left += 1;
        } else {
            area = (right - left) as i32 * height[right];
            right -= 1;
        }
        max_area = area.max(max_area);
    }

    max_area
}

fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let out = max_area(height);
    assert_eq!(out, 49);

    let height = vec![1, 1];
    let out = max_area(height);
    assert_eq!(out, 1);
}
