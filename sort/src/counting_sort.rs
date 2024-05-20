// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::ops::Sub;

pub fn counting_sort_generic<T>(nums: &mut [T])
where
    T: Copy + Default + Ord + Sub<Output = T> + Into<usize>,
{
    if nums.is_empty() {
        return;
    }
    let min_num: T = nums.iter().min().copied().unwrap_or_default();
    let max_num: T = nums.iter().max().copied().unwrap_or_default();
    let range: T = max_num - min_num;
    let size: usize = range.into();
    // 构造计数数组
    let mut counts = vec![0_usize; size];

    // 遍历数组, 更新计数数组
    for num in nums.iter() {
        let delta: T = *num - min_num;
        let index: usize = delta.into();
        counts[index] += 1;
    }

    // 生成累积数组
    for i in 1..size {
        counts[i] += counts[i - 1];
    }

    // 反向填充目标数组
    let len = nums.len();
    let mut mirror: Vec<T> = Vec::new();
    mirror.extend_from_slice(nums);

    for i in (0..len).rev() {
        let num: T = mirror[i];
        let diff: T = num - min_num;
        let index: usize = counts[diff.into()];
        // 把 num 放在对应的位置
        nums[index - 1] = num;

        // 同时更新 counts
        counts[index] -= 1;
    }
}

#[allow(clippy::cast_sign_loss)]
pub fn counting_sort(nums: &mut [i32]) {
    if nums.is_empty() {
        return;
    }
    let min_num: i32 = nums.iter().min().copied().unwrap_or_default();
    let max_num: i32 = nums.iter().max().copied().unwrap_or_default();
    let range: i32 = max_num - min_num;
    let size: usize = range as usize + 1;
    // 构造计数数组
    let mut counts = vec![0_usize; size];

    // 遍历数组, 更新计数数组
    for num in nums.iter() {
        let delta: i32 = *num - min_num;
        let index: usize = delta as usize;
        counts[index] += 1;
    }

    // 生成累积数组
    for i in 1..size {
        counts[i] += counts[i - 1];
    }

    // 反向填充目标数组
    let len = nums.len();
    let mut mirror: Vec<i32> = Vec::new();
    mirror.extend_from_slice(nums);

    for i in (0..len).rev() {
        let num: i32 = mirror[i];
        let diff: i32 = num - min_num;
        let index: usize = counts[diff as usize];
        // 把 num 放在对应的位置
        nums[index - 1] = num;

        // 同时更新 counts
        // FIXME(Shaohua): subtract overflow
        //counts[index] -= 1;
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
