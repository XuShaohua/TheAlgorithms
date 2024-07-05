// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#[inline]
pub fn quicksort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    if arr.len() < 2 {
        return;
    }
    quicksort_helper(arr, 0, arr.len() - 1);
}

fn quicksort_helper<T>(arr: &mut [T], low: usize, high: usize)
where
    T: PartialOrd,
{
    if low >= high {
        return;
    }

    // 按照基数的位置, 将数组划分成左右两个子数组.
    let pivot_index = partition(arr, low, high);
    // 对左右两个子数组分别递归进行快速排序
    if pivot_index > low {
        quicksort_helper(arr, low, pivot_index - 1);
    }
    if pivot_index + 1 < high {
        quicksort_helper(arr, pivot_index + 1, high);
    }
}

fn partition<T>(arr: &mut [T], low: usize, high: usize) -> usize
where
    T: PartialOrd,
{
    // 选择最左侧的元素作为基准值
    let pivot_index = low;

    // 使用双指针法遍历数组, 以 pivot 为基准, 把数组划分成三部分:
    // 小于等于 pivot, pivot, 大于等于 pivot
    let mut left: usize = low + 1;
    let mut right: usize = high;
    while left < right {
        // left 的位置右移, 但是确保 arr[left] 小于 pivot
        while left < right && arr[left] <= arr[pivot_index] {
            left += 1;
        }
        // right 的位置左移, 但确保 arr[right] 大于 pivot
        while left < right && arr[right] >= arr[pivot_index] {
            right -= 1;
        }
        // 交换元素
        arr.swap(left, right);
    }
    // 最后把基准值 pivot 移到合适的位置.
    // 此时, 数组中元素的顺序满足以下条件: 小于等于 pivot, pivot, 大于等于 pivot
    arr.swap(left, pivot_index);
    // 返回的是 pivot 所在的位置
    left
}

#[cfg(test)]
mod tests {
    use crate::quicksort::quicksort;

    #[test]
    fn test_quicksort() {
        let mut list = [0, 5, 3, 2, 2];
        quicksort(&mut list);
        assert_eq!(list, [0, 2, 2, 3, 5]);

        let mut list = [-2, -5, -45];
        quicksort(&mut list);
        assert_eq!(list, [-45, -5, -2]);

        let mut list = [
            -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -983_833,
            -987_905, -980_069, -977_640,
        ];
        quicksort(&mut list);
        assert_eq!(
            list,
            [
                -998_166, -996_360, -995_703, -995_238, -995_066, -994_740, -992_987, -987_905,
                -983_833, -980_069, -977_640,
            ]
        );

        let mut list = "EASYQUESTION".chars().collect::<Vec<_>>();
        quicksort(&mut list);
        assert_eq!(
            list,
            ['A', 'E', 'E', 'I', 'N', 'O', 'Q', 'S', 'S', 'T', 'U', 'Y']
        );
    }
}