// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use crate::insertion_sort::insertion_sort;
use crate::shell_sort::shell_sort;

/// 桶排序, 使用插入排序来处理每个桶.
#[allow(clippy::cast_sign_loss)]
pub fn bucket_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    // 对于插入排序来说, 元素的个数在这个范围内的效率比较高.
    let bucket_elements: usize = 72;
    let min_num: i32 = arr.iter().min().copied().unwrap_or_default();
    let max_num: i32 = arr.iter().max().copied().unwrap_or_default();
    // 计算数值范围.
    let range: i32 = max_num - min_num;
    // 计算桶的个数, 我们假设元素的数值是均匀分布的.
    // 这样的话就可以确定每个桶要存储的数值范围.
    // 尽可能把数值相近的元素放在一起.
    let bucket_count: usize = range as usize / bucket_elements + 1;
    // 创建一系列的桶.
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; bucket_count];

    // 遍历数组, 将元素分配到每个桶中.
    // 这里是按数组的原有顺序插入到桶中的, 有相同数值的元素也会依照原先的顺序放置到同一个桶.
    for &num in arr.iter() {
        // 计算这个元素值处于哪个数值段, 并确定该放到哪个桶.
        let bucket_index: usize = (num - min_num) as usize / bucket_elements;
        buckets[bucket_index].push(num);
    }

    // 对每一个桶单独排序, 按照假设, 每个桶中的元素个数都比较少,
    // 使用插入排序可以发挥它的优势.
    // 并且插入排序是稳定排序, 所以该桶排序算法也是稳定排序.
    let mut index: usize = 0;
    for mut bucket in buckets {
        insertion_sort(&mut bucket);
        // 将这个桶中的元素合并到原先的数组中.
        arr[index..(index + bucket.len())].copy_from_slice(&bucket);
        index += bucket.len();
    }
}

/// 桶排序的另一种实现, 使用希尔排序来处理每个桶.
#[allow(clippy::cast_sign_loss)]
pub fn shell_bucket_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    // 对于希尔排序来说, 元素的个数在这个范围内的效率比较高.
    let bucket_elements: usize = 72 * 3;

    let min_num: i32 = arr.iter().min().copied().unwrap_or_default();
    let max_num: i32 = arr.iter().max().copied().unwrap_or_default();
    let range: i32 = max_num - min_num;
    let bucket_count: usize = range as usize / bucket_elements + 1;
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; bucket_count];

    for &num in arr.iter() {
        let bucket_index: usize = (num - min_num) as usize / bucket_elements;
        buckets[bucket_index].push(num);
    }
    let mut index: usize = 0;
    for mut bucket in buckets {
        shell_sort(&mut bucket);
        arr[index..(index + bucket.len())].copy_from_slice(&bucket);
        index += bucket.len();
    }
}

#[cfg(test)]
mod tests {
    use super::{bucket_sort, shell_bucket_sort};

    #[test]
    fn test_bucket_sort() {
        let mut list = [0, 5, 3, 2, 2];
        bucket_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        bucket_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        bucket_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );
    }

    #[test]
    fn test_shell_bucket_sort() {
        let mut list = [0, 5, 3, 2, 2];
        shell_bucket_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        shell_bucket_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        shell_bucket_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );
    }
}
