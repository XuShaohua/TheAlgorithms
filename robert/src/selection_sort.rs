// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::generics;

/// 即使输入数据已经是排好序的, 该算法依然需要 N^2 次的操作.
/// N^2 / 2 次比较以及 N 次交换.
pub fn selection_sort<T>(vec: &mut Vec<T>)
where
    T: Clone + PartialOrd + std::fmt::Display,
{
    let n = vec.len();
    for i in 0..n {
        let mut min = i;
        for j in (i + 1)..n {
            if vec[j] < vec[min] {
                min = j;
            }
        }
        if i != min {
            generics::exch(vec, i, min);
        }
        generics::show(&vec);
    }
}
