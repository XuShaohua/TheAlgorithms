// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max_count = 0;
    let mut count_of_1 = 0;
    for num in &nums {
        if *num == 1 {
            count_of_1 += 1;
        } else {
            max_count = max_count.max(count_of_1);
            count_of_1 = 0;
        }
    }
    max_count.max(count_of_1)
}

fn check_solution() {
    let nums = vec![1, 1, 0, 1, 1, 1];
    assert_eq!(find_max_consecutive_ones(nums), 3);
    let nums = vec![1, 0, 1, 1, 0, 1];
    assert_eq!(find_max_consecutive_ones(nums), 2);
}

fn main() {
    check_solution();
}
