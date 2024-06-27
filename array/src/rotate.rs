// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

/// 使用临时数组
pub fn rotate_left_1(slice: &mut [i32], k: usize) {
    if slice.is_empty() {
        return;
    }

    let len = slice.len();
    let k = k % len;
    if k == 0 {
        return;
    }
    debug_assert!(k > 0 && k < len);

    let mut tmp: Vec<i32> = Vec::with_capacity(len);
    // 复制第一部分
    for &num in &slice[k..] {
        tmp.push(num);
    }

    // 复制第二部分
    for &num in &slice[..k] {
        tmp.push(num);
    }

    // 写回到原数组
    for (i, &num) in tmp.iter().enumerate() {
        slice[i] = num;
    }
}

/// 原地反转数组
pub fn rotate_left_2(slice: &mut [i32], k: usize) {
    if slice.is_empty() {
        return;
    }

    let len = slice.len();
    let k = k % len;
    if k == 0 {
        return;
    }
    debug_assert!(k > 0 && k < len);

    slice[k..len].reverse();
    slice[..k].reverse();
    slice.reverse();
}

#[cfg(test)]
mod tests {
    use super::{rotate_left_1, rotate_left_2};

    #[test]
    fn test_rotate_left_1() {
        let mut arr = [1, 2, 3, 4];
        rotate_left_1(&mut arr, 6);
        assert_eq!(arr, [3, 4, 1, 2]);

        let mut arr = [1, 2, 3, 4];
        rotate_left_1(&mut arr, 1);
        assert_eq!(arr, [2, 3, 4, 1]);
    }

    #[test]
    fn test_rotate_left_2() {
        let mut arr = [1, 2, 3, 4];
        rotate_left_2(&mut arr, 6);
        assert_eq!(arr, [3, 4, 1, 2]);

        let mut arr = [1, 2, 3, 4];
        rotate_left_2(&mut arr, 1);
        assert_eq!(arr, [2, 3, 4, 1]);
    }
}
