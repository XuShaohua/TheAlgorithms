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
    let arr = [10, 4, 16, 20];
    let ps = prefix_sum(&arr);
    println!("prefix sum array: {ps:?}");
}