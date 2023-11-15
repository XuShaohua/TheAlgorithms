// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::generics;

/// 拆解成由 h 个元素隔开的序列, 依次降低h间隔的值, 直到其为1.
/// 主要是为了减少元素交换的次数.
/// 最差情况下 O(N^(3/2))
pub fn shell_sort<T>(vec: &mut Vec<T>)
where
    T: Clone + PartialOrd + std::fmt::Display,
{
    let n = vec.len();
    let mut h = 1;
    while h * 3 < n {
        h = 3 * h + 1;
    }
    println!("h0 = {}", h);

    while h >= 1 {
        // h-sort the array.
        for i in h..n {
            for j in (h..=i).rev().step_by(h) {
                if vec[j - h] > vec[j] {
                    generics::exch(vec, j - h, j);
                    generics::show(&vec);
                } else {
                    break;
                }
            }
        }
        h = h / 3;
        println!("h = {}", h);
    }
}
