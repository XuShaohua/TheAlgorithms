// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::generics;

/// 如果传数的数据是增序排好的, 那么只需要 N-1 次的比较, 以及 0 次的交换;
/// 如果传数的数据是降序排好的, 那么需要 N^2/2 次的比较, 以及 N^2/2 次的交换;
/// 如果是乱序的, 大概需要 N^2/4 次的比较, 以及 N^2/4 次的交换.
/// 其思路是, 先将前 i 个元素调整为增序的, 随着 i 从 0 增大到 n, 整个序列
/// 就变得是增序了.
pub fn insertion_sort_internal<T>(vec: &mut Vec<T>, low: usize, high: usize)
where
    T: Clone + PartialOrd + std::fmt::Display,
{
    for i in (low + 1)..(high + 1) {
        for j in ((low + 1)..=i).rev() {
            if vec[j - 1] > vec[j] {
                generics::exch(vec, j - 1, j);
                generics::show(&vec);
            } else {
                break;
            }
        }
    }
}

pub fn insertion_sort<T>(vec: &mut Vec<T>)
where
    T: Clone + PartialOrd + std::fmt::Display,
{
    let len = vec.len();
    insertion_sort_internal(vec, 0, len - 1);
}
