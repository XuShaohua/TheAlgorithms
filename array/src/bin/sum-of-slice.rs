// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

fn prefix_sum(numbers: &[i32]) -> Vec<i32> {
    let mut accum = 0;
    let mut ps = Vec::with_capacity(numbers.len());
    for num in numbers {
        accum += num;
        ps.push(accum);
    }
    ps
}

fn main() {
    let arr = [8, 19, 28, 21, 33, 97, 62, 7, 45];
    let ps = prefix_sum(&arr);
    for left in 0..2 {
        for right in 3..arr.len() {
            let sum = if left == 0 {
                ps[right]
            } else {
                ps[right] - ps[left - 1]
            };
            let sum_slice = arr[left..=right].iter().sum();
            assert_eq!(sum, sum_slice);
        }
    }
}
