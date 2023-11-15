// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::generics;

pub fn shuffle_sort<T>(vec: &mut Vec<T>)
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
