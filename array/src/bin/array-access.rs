// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

fn main() {
    let numbers: [i32; 6] = [1, 1, 2, 3, 5, 8];
    assert_eq!(numbers[0], 1);
    assert_eq!(numbers[3], 3);
    assert_eq!(numbers[5], 8);
}