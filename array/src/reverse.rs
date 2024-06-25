// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::manual_swap)]

pub fn reverse_array(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }

    let mut start = 0;
    let mut end = arr.len() - 1;
    while start < end {
        let tmp = arr[end];
        arr[end] = arr[start];
        arr[start] = tmp;
        start += 1;
        end -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::reverse_array;

    #[test]
    fn test_reverse_array() {
        let mut arr = [1, 2, 3, 4, 5];
        reverse_array(&mut arr);
        assert_eq!(arr, [5, 4, 3, 2, 1]);
    }
}
