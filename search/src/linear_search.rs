// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[must_use]
pub fn linear_search<T: PartialOrd>(slice: &[T], target: &T) -> Option<usize> {
    for (index, item) in slice.iter().enumerate() {
        if item == target {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::linear_search;

    #[test]
    fn test_linear_search() {
        let arr = [10, 50, 30, 70, 80, 60, 20, 90, 40];
        let target = 30;
        let index = linear_search(&arr, &target);
        assert_eq!(index, Some(2));
    }
}
