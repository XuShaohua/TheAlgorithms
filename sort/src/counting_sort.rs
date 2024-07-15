// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::ops::Sub;

pub fn counting_sort_generic<T>(arr: &mut [T])
where
    T: Copy + Default + Ord + Sub<Output=T> + Into<usize>,
{
    if arr.is_empty() {
        return;
    }
    let min_num: T = arr.iter().min().copied().unwrap_or_default();
    let max_num: T = arr.iter().max().copied().unwrap_or_default();
    let range: T = max_num - min_num;
    let size: usize = range.into();
    // 构造计数数组
    let mut counts = vec![0_usize; size];

    // 遍历数组, 更新计数数组
    for num in arr.iter() {
        let delta: T = *num - min_num;
        let index: usize = delta.into();
        counts[index] += 1;
    }

    // 生成累积数组
    for i in 1..size {
        counts[i] += counts[i - 1];
    }

    // 反向填充目标数组
    let len = arr.len();
    let mut mirror: Vec<T> = Vec::new();
    mirror.extend_from_slice(arr);

    for i in (0..len).rev() {
        let num: T = mirror[i];
        let diff: T = num - min_num;
        let index: usize = counts[diff.into()];
        // 把 num 放在对应的位置
        arr[index - 1] = num;

        // 同时更新 counts
        counts[index] -= 1;
    }
}

#[allow(clippy::cast_sign_loss)]
pub fn counting_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }
    let min_num: i32 = arr.iter().min().copied().unwrap_or_default();
    let max_num: i32 = arr.iter().max().copied().unwrap_or_default();
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
    let len = arr.len();

    // 从输入数组的右侧向左侧遍历, 这样实现的是稳定排序.
    for i in (0..len).rev() {
        let num: i32 = input_arr[i];
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

#[cfg(test)]
mod tests {
    use super::counting_sort;

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
}
