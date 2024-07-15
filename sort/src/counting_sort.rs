// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::collections::BTreeMap;
use std::ops::Sub;

pub fn counting_sort_generic<T>(arr: &mut [T])
where
    T: Copy + Default + Ord + Sub<Output=T> + TryInto<usize>,
{
    if arr.is_empty() {
        return;
    }
    let min_num: T = arr.iter().min().copied().unwrap_or_default();
    let max_num: T = arr.iter().max().copied().unwrap_or_default();
    // 计算数值范围
    let range: T = max_num - min_num;
    let size: usize = range.try_into().unwrap_or_default() + 1;

    // 构造计数数组
    let mut count_arr = vec![0_usize; size];

    // 遍历数组, 更新计数数组
    for num in arr.iter() {
        let delta: T = *num - min_num;
        let index: usize = delta.try_into().unwrap_or_default();
        count_arr[index] += 1;
    }

    // 生成累积数组, prefix sum array
    for i in 1..size {
        count_arr[i] += count_arr[i - 1];
    }

    // 构造输入数组, 只读的
    let input_arr = arr.to_vec();

    for &num in input_arr.iter().rev() {
        let delta: T = num - min_num;
        let delta_index: usize = delta.try_into().unwrap_or_default();
        // 从 count_arr 里取出该数值的相对位置
        let num_index: usize = count_arr[delta_index];
        // 把 num 放在对应的位置
        arr[num_index - 1] = num;

        // 同时更新 count_arr, 使之计数减1, 这样的话下一个相同数值的元素的索引值就被左移了一位.
        count_arr[delta_index] -= 1;
    }
}

#[allow(clippy::cast_sign_loss)]
pub fn counting_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }
    let min_num: i32 = arr.iter().min().copied().unwrap_or_default();
    let max_num: i32 = arr.iter().max().copied().unwrap_or_default();
    // 计算数值范围
    let range: i32 = max_num - min_num;
    let size: usize = range as usize + 1;

    // 构造计数数组
    let mut count_arr = vec![0_usize; size];

    // 遍历输入数组, 更新计数数组
    for &num in arr.iter() {
        let delta: i32 = num - min_num;
        let index: usize = delta as usize;
        count_arr[index] += 1;
    }

    // 生成累积数组, prefix sum array
    for i in 1..size {
        count_arr[i] += count_arr[i - 1];
    }

    // 构造输入数组, 只读的
    let input_arr: Vec<i32> = arr.to_vec();

    // 从输入数组的右侧向左侧遍历, 这样实现的是稳定排序.
    for &num in input_arr.iter().rev() {
        // 计算当前值与最小值的差.
        let delta: i32 = num - min_num;
        let delta_index = delta as usize;
        // 从 count_arr 里取出该数值的相对位置
        let num_index: usize = count_arr[delta_index];
        // 把 num 放在对应的位置
        arr[num_index - 1] = num;

        // 同时更新 count_arr, 使之计数减1, 这样的话下一个相同数值的元素的索引值就被左移了一位.
        count_arr[delta_index] -= 1;
    }
}


#[allow(clippy::cast_sign_loss)]
pub fn counting_sort_with_map(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    // 构造字典, 存储元素的频率
    let mut freq_map: BTreeMap<i32, usize> = BTreeMap::new();
    // 遍历输入数组, 更新计数数组
    for &num in arr.iter() {
        *freq_map.entry(num).or_default() += 1;
    }

    // 遍历字典
    let mut i = 0;
    for (num, freq) in freq_map {
        for _j in 0..freq {
            arr[i] = num;
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{counting_sort, counting_sort_generic, counting_sort_with_map};

    #[test]
    fn test_counting_sort() {
        let mut list = [0, 5, 3, 2, 2];
        counting_sort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        counting_sort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        counting_sort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );
    }

    #[test]
    fn test_counting_sort_generic() {
        let mut list = [0, 5, 3, 2, 2];
        counting_sort_generic(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        counting_sort_generic(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        counting_sort_generic(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );
    }

    #[test]
    fn test_counting_sort_with_map() {
        let mut list = [0, 5, 3, 2, 2];
        counting_sort_with_map(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        counting_sort_with_map(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        counting_sort_with_map(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );
    }
}
