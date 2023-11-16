// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

/// Place one element at a time, inserting each into its proper place.
///
/// The elements to the left of the current index are in sorted order during
/// the sort, but they are not in their final positions, as they may have to
/// be moved to make room for smaller elements later.
///
/// 其思路是, 先将前 i 个元素调整为增序的, 随着 i 从 0 增大到 n, 整个序列
/// 就变得是增序了.
///
/// 如果传数的数据是增序排好的, 那么只需要 N-1 次的比较, 以及 0 次的交换;
/// 如果传数的数据是降序排好的, 那么需要 N^2/2 次的比较, 以及 N^2/2 次的交换;
/// 如果是乱序的, 大概需要 N^2/4 次的比较, 以及 N^2/4 次的交换.
pub fn insertion_sort<T>(list: &mut [T])
where
    T: PartialOrd,
{
    let len = list.len();
    for i in 1..len {
        for j in (1..=i).rev() {
            if list[j - 1] > list[j] {
                list.swap(j - 1, j);
            } else {
                break;
            }
        }
    }
}
