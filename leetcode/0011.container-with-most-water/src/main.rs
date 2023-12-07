// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

pub fn max_area(height: Vec<i32>) -> i32 {
    let len = height.len();
    assert!(len > 1);

    let mut max_area = 0;
    for i in 0..(len - 1) {
        for j in (i + 1)..len {
            let min_height = height[i].min(height[j]);
            let area = min_height * (j - i) as i32;
            if max_area < area {
                max_area = area;
            }
        }
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
